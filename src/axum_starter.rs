use axum_starter::{prepare, state::AddState};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlConfig {
    #[serde(alias = "db_url")]
    url: url::Url,
}

#[prepare(ConnectSQL?)]
pub async fn connect_sql_db(cfg: &SqlConfig) -> Result<AddState<DatabaseConnection>, DbErr> {
    let conn = Database::connect(ConnectOptions::new(cfg.url.to_string())).await?;
    Ok(AddState::new(conn))
}
