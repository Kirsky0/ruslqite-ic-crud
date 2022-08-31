use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct User {
    pub userid: String,
    pub name: String,

}