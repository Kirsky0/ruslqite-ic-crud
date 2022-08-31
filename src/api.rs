use std::sync::Mutex;

use candid::candid_method;
use candid::CandidType;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use rusqlite::{Connection, Transaction, Error};
use serde::{Deserialize, Serialize};

use crate::types::User;


static CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE  IF NOT EXISTS user (
            userid    CHARACTER(255) PRIMARY KEY,
            name  CHARACTER(255) NOT NULL
        )",
        (), // empty list of parameters.
    );
    Mutex::new(conn)
});


#[query(name = "mem_page")]
#[candid_method(query)]
pub async fn mem_page() -> u64 {
    return super::mem::get_heap_memory_size();
}


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}


#[query(name = "list")]
#[candid_method(query)]
pub async fn list() -> String {
    let conn = CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM user  ").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut users = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let p = User {
            userid: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        };
        users.push(p)
    }
    let json_str = serde_json::to_string(&users);


    return json_str.unwrap();
}


#[update(name = "data_add")]
#[candid_method(update)]
pub async fn data_add() {
    // let u = User {
    //     userid: "user1".to_string(),
    //     name: "ttt".to_string(),
    // };
    // let conn = CONN.lock().unwrap();
    //
    // let r = conn.execute(
    //     "INSERT INTO user (userid, name) VALUES (?1, ?2)",
    //     (&u.userid, &u.name),
    // );
    let data = r#"
    [{
	"userid": "user1",
	"name": "personal"
}, {
	"userid": "user2",
	"name": "business"
}]
    "#;

    let users: Vec<User> = serde_json::from_str(data).unwrap();

    let mut conn = CONN.lock().unwrap();

    let tx = conn.transaction().unwrap();
    {
        let mut stmt = tx.prepare("INSERT INTO user (userid, name) VALUES (?1, ?2)").unwrap();
        for user in users
        {
            stmt.insert((user.userid, user.name));
        }
    }
    tx.commit();
}


