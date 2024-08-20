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
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i32,
    pub block: i32,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Pair {
    pub token0: String,
    pub token1: String,
    pub pair: String,
    pub timestamp: i32,
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = pairs)]
pub struct NewPair {
    pub token0: String,
    pub token1: String,
    pub pair: String,
    pub timestamp: i32,
    pub block: i32,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Trade {
    pub token_in: String,
    pub token_out: String,
    pub amount_in: i32,
    pub amount_out: i32,
    pub fees: i32,
    pub change : i32,
    pub timestamp: i32,
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub token_in: String,
    pub token_out: String,
    pub amount_in: i32,
    pub amount_out: i32,
    pub fees: i32,
    pub change : i32,
    pub timestamp: i32,
    pub block: i32,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Price {
    pub pair: i32,
    pub price: i32,
    pub timestamp: i32,
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = prices)]
pub struct NewPrice {
    pub pair: i32,
    pub price: i32,
    pub timestamp: i32,
    pub block: i32,
}