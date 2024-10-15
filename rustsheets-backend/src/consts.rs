use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Type, Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TimestampType {
    Started,
    Finished,
    Paused,
    Resumed,
}

#[derive(Type, Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Copy)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Blockchain {
    Ethereum,
}
