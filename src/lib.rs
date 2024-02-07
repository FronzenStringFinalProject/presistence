pub mod operations;
pub mod service;
pub use sea_orm;
pub mod entities;
mod axum_starter;

pub use axum_starter::{ConnectSQL,SqlConfig};
