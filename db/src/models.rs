use diesel::{prelude::*};
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
use bigdecimal::BigDecimal;
use diesel::data_types::PgNumeric;
use crate::schema::{tokens, pairs, trades, prices};
// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName, Insertable)]
pub struct Token {
    pub id: Option<i32>,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
pub struct Pair {
    pub id: i32,
    pub token0: String,
    pub token1: String,
    pub pair: String,
    pub block: i32,
}

#[derive(Insertable)]
#[diesel(table_name = pairs)]
pub struct NewPair {
    pub token0: String,
    pub token1: String,
    pub pair: String,
    pub block: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
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

#[derive(Insertable)]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub token_in: String,
    pub token_out: String,
    pub amount_in: i64,
    pub amount_out: i64,
    pub fees: i64,
    pub change : BigDecimal,
    pub block: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
pub struct Price {
    pub id: i32,
    pub pair: String,
    pub price: f64,
    pub block: i32,
}

#[derive(Insertable)]
#[diesel(table_name = prices)]
pub struct NewPrice {
    pub pair: String,
    pub price: BigDecimal,
    pub block: i32,
}