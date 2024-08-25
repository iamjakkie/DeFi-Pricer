pub mod connection;
pub mod models;
// pub mod operations;
pub mod schema;

use crate::schema::{tokens, pairs, trades};
use crate::connection::establish_connection_pg;