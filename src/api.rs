use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Mutex;

use candid::candid_method;
use candid::CandidType;
use ic_cdk::api::call::{call, CallResult};
use ic_cdk::api::caller;
use ic_cdk::api::time;
use ic_cdk::export::Principal;
use ic_cdk::id;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::manage_canister::types::*;

static CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open_in_memory().unwrap();
    Mutex::new(conn)
});

#[query(name = "canister_size")]
#[candid_method(query)]
pub async fn canister_size() -> u64 {
    return super::mem::get_heap_memory_size();
    // return std::mem::size_of::<u64>() as u64;
}

#[query(name = "page")]
#[candid_method(query)]
pub async fn page() -> bool {
    let bool = wasmtime::MemoryType::is_64(Self);
    return bool;
}


#[update(name = "update_setting")]
#[candid_method(update)]
pub async fn update_setting() {
    let setting = CanisterSettings {
        controllers: Some(vec![id(), Principal::management_canister(), Principal::from_text("w6sok-rucwt-qaqeq-zalht-k7yva-cvtji-vx5ma-tl3nl-67sbr-3bvq5-aqe").unwrap()]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    };
    let update_setting = UpdateSettings {
        canister_id: id(),
        settings: setting,
    };

    let r: CallResult<((), )> = call(
        Principal::management_canister(),
        "update_settings",
        (update_setting, ),
    ).await;
    if let Err((code, msg)) = r {
        ic_cdk::api::trap(&msg);
    }
}

#[update(name = "size")]
#[candid_method(update)]
pub async fn size() -> String {
    let id = CanisterId {
        canister_id: id(),
    };
    let r: CallResult<(CanisterStatus, )> = call(
        Principal::management_canister(),
        "canister_status",
        (id, ),
    ).await;
    if let Err((code, msg)) = r {
        ic_cdk::api::trap(&msg);
    }
    let status = r.unwrap().0;
    return serde_json::to_string(&status).unwrap();
    // call(Principal::manage_canister(), "canister_status", ()).await;
}


#[query(name = "nano_time")]
#[candid_method(query)]
pub async fn nano_time() -> u64 {
    return time();
}


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}


#[update(name = "data_add")]
#[candid_method(update)]
pub async fn data_add() {
    // let conn = Connection::open_in_memory().unwrap();

    CONN.lock().unwrap().execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    );
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    CONN.lock().unwrap().execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    );
}

#[query(name = "data_get")]
#[candid_method(query)]
pub async fn data_get() -> String {
    let conn = CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM person where name=? ").unwrap();
    let name = "Steven";
    let mut rows = stmt.query(rusqlite::params![name]).unwrap();

    let mut persons = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let p = Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            data: row.get(2).unwrap(),
        };
        persons.push(p)
    }

    let json_str = serde_json::to_string(&persons);
    return json_str.unwrap();
}