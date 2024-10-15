use crate::consts::{Blockchain, TimestampType};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,

    pub name: Option<String>,
    pub surname: Option<String>,

    pub payable_hour_rate: f64,
    pub telegram: Option<String>,
    pub discord: Option<String>,

    pub owed: f64,
}

#[derive(FromRow, Debug, Clone)]
pub struct Wallet {
    pub wallet_address: String,
    pub blockchain: Blockchain,
    pub is_active: bool,

    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub client_name: String,
    pub description: String,
    pub billable_rate: f64,
    pub active: bool,
    pub executor_id: Uuid,
}

#[derive(FromRow, Debug, Clone)]
pub struct PersonalRate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub payable_rate: f64,
    pub billable_rate: f64,
}

#[derive(FromRow, Debug, Clone)]
pub struct WorkSession {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub closed: bool,
}

#[derive(FromRow, Debug, Clone)]
pub struct WorkDay {
    pub id: Uuid,
    pub date: NaiveDate,
    pub work_session_id: Uuid,
}

#[derive(FromRow, Debug, Clone)]
pub struct Record {
    pub id: Uuid,
    pub user_id: Uuid,
    pub work_day_id: Uuid,
    pub project_id: Uuid,
    pub payable_earnings: f64,
    pub billable_earnings: f64,
    pub summary: String,
    pub finished: bool,
}

#[derive(FromRow, Debug, Clone)]
pub struct RecordTimestamp {
    pub id: Uuid,
    pub record_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub timestamp_type: TimestampType,
}

#[derive(FromRow, Debug, Clone)]
pub struct ProjectInvoice {
    pub id: Uuid,
    pub session_id: Uuid,
    pub project_id: Uuid,
    pub date: NaiveDate,
    pub invoice_pdf_path: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Executor {
    pub id: Uuid,
    pub name: String,
}
