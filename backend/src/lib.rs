#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

pub mod cli;
pub mod api;
pub mod models;
pub mod middleware;
pub mod database;
pub mod error;
pub mod authentication;
pub mod response;
pub mod storage;
pub mod helpers;