use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub userid: String,
    pub name: String,

}

#[derive(CandidType, Debug, Serialize, Deserialize, Clone)]
pub struct UserBatch {
    pub userid: String,
    pub name: String,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
    pub t5: String,
    pub t6: String,
    pub t7: String,
    pub t8: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub code: u64,
    pub msg: String,
}