use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Token {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block: i32,
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Pair {
    pub id: i32,
    pub token0: String,
    pub token1: String,
    pub pair: String,
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = pairs)]
pub struct NewPair {
    pub token0: String,
    pub token1: String,
    pub pair: String,
    pub block: i32,
}

#[derive(Queryable, Serialize, PartialEq, PartialOrd)]
pub struct Trade {
    pub id: i32,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: i64,
    pub amount_out: i64,
    pub fees: i64,
    pub change : f64,
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
    pub change : f64,
    pub block: i32,
}

#[derive(Queryable, Serialize, PartialEq, PartialOrd)]
pub struct Price {
    pub id: i32,
    pub pair: i32,
    pub price: f64,
    pub block: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = prices)]
pub struct NewPrice {
    pub pair: i32,
    pub price: f64,
    pub block: i32,
}