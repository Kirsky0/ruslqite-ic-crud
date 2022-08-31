use std::collections::LinkedList;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};


fn main()
{
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE user (
            userid    CHARACTER(255) PRIMARY KEY,
            name  CHARACTER(255) NOT NULL,
        )",
        (), // empty list of parameters.
    );

    // let data = r#"
    // [
    //   {
    //     "userid": 1,
    //     "name": "personal",
    //     "avatar":"",
    //     "address" :""
    //   },
    //   {
    //     "userid": 2,
    //     "name": "business",
    //     "avatar":"",
    //     "address":""
    //   }
    // ]
    // "#;
    //
    // let profiles: Vec<User> = serde_json::from_str(data).unwrap();
    //
    //
    // println!("{:#?}", profiles);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub userid: u32,
    pub name: String,
    pub avatar: String,
    pub address: String,

}