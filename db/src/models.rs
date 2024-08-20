use crate::schema::posts;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i32,
}
