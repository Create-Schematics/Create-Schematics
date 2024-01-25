#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

pub mod api;
pub mod authentication;
pub mod cli;
pub mod database;
pub mod error;
pub mod helpers;
pub mod middleware;
pub mod models;
pub mod redirect;
pub mod response;
pub mod storage;
