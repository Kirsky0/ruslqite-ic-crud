use std::thread;
use std::any::Any;
use std::os::raw::c_int;
use std::sync::{Mutex, MutexGuard};

use candid::candid_method;
use candid::CandidType;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use once_cell::sync::Lazy;
use rusqlite::{Batch, Connection, Error, Result, trace, Transaction};
use serde::{Deserialize, Serialize};

use crate::types::{Log, User, UserBatch};

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

static LOGS: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| {
    let logs: Vec<Log> = Vec::new();
    Mutex::new(logs)
});


#[init]
fn init()
{
    unsafe
        {
            trace::config_log(Some(save_log));
        }
}

fn save_log(core: c_int, msg: &str)
{
    let log = Log { code: (core as u64), msg: msg.to_string() };
    LOGS.lock().unwrap().push(log);
}

#[query(name = "mem_page")]
#[candid_method(query)]
pub async fn mem_page() -> u64 {
    return super::mem::mem_page_size();
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


#[query(name = "log")]
#[candid_method(query)]
pub async fn log() -> String
{
    let logs = LOGS.lock().unwrap();
    let unlock_logs = logs.to_vec();
    let json_str = serde_json::to_string(&unlock_logs).unwrap();
    return json_str;
}

#[query(name = "looper_size")]
#[candid_method(query)]
pub fn looper_size() -> i32 {
    let conn = CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM test  ").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut size: i32 = 0;
    while let Some(row) = rows.next().unwrap() {
        size = row.get_unwrap(0);
    }
    return size;
}


#[update(name = "create_test")]
#[candid_method(update)]
pub async fn create_test() {
    let mut conn = CONN.lock().unwrap();
    conn.execute(
        "CREATE TABLE  IF NOT EXISTS test (
            userid  INTEGER PRIMARY KEY,
            name  CHARACTER(255) NOT NULL,

            t1  CHARACTER(255) NOT NULL,
            t2  CHARACTER(255) NOT NULL,
            t3  CHARACTER(255) NOT NULL,
            t4  CHARACTER(255) NOT NULL,
            t5  CHARACTER(255) NOT NULL,
            t6  CHARACTER(255) NOT NULL,
            t7  CHARACTER(255) NOT NULL,
            t8  CHARACTER(255) NOT NULL
        )",
        (), // empty list of parameters.
    );
}

#[update(name = "execute_test")]
#[candid_method(update)]
pub async fn execute_test(start: i32, range: i32) {
    let mut conn = CONN.lock().unwrap();

    let sql: &str = "INSERT INTO test (userid, name,t1,t2,t3,t4,t5,t6,t7,t8) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9 , ?10)";
    let u = UserBatch {
        userid: "user1".to_string(),
        name: "ttt".to_string(),
        t1: "a".to_string(),
        t2: "a".to_string(),
        t3: "a".to_string(),
        t4: "a".to_string(),
        t5: "a".to_string(),
        t6: "a".to_string(),
        t7: "a".to_string(),
        t8: "a".to_string(),
    };

    let tx = conn.transaction().unwrap();
    {
        let mut stmt = tx.prepare(sql).unwrap();

        let zero_vec = vec![0; range as usize];

        for (index, value) in zero_vec.iter().enumerate() {
            let user_id = index as i32 + start;
            stmt.insert(rusqlite::params![user_id,&u.name, &u.t1, &u.t2, &u.t3, &u.t4, &u.t5, &u.t6, &u.t7, &u.t8]);
        }
    }
    tx.commit();
}


#[query(name = "test_table_size")]
#[candid_method(query)]
pub fn test_table_size() -> i32 {
    let conn = CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT SUM(pgsize) FROM dbstat WHERE name='test' ").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut size: i32 = 0;
    while let Some(row) = rows.next().unwrap() {
        size = row.get_unwrap(0);
    }
    return size;
}


#[query(name = "test_delete")]
#[candid_method(query)]
pub fn test_delete(row: i32) {
    let conn = CONN.lock().unwrap();
    let sql: &str = "DELETE FROM test WHERE userid IN (SELECT userid FROM test ORDER BY  userid DESC LIMIT ?1);";
    conn.execute(sql, rusqlite::params![row]);
}

