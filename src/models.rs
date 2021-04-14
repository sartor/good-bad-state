use serde::Serialize;
use sqlx::{FromRow};
use chrono::NaiveDateTime;

#[derive(FromRow, Debug, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}
