pub mod operations;
pub mod service;
pub use sea_orm;
mod axum_starter;
pub mod entities;

pub use axum_starter::{ConnectSQL, SqlConfig};
