use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize)]
pub struct RequiredIterationFields {
    pub title: String,
    pub goals: String,
    pub created_by: i32,
    pub participants: Vec<i32>,
    pub created_date: i64,
    pub end_date: i64,
}

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct RequiredBacklogFields {
    pub title: String,
    pub goals: String,
    pub description: String,
    pub priority: i32,
    pub hours: i32,
    pub points: i32,
    pub created_date: i64,
    pub iteration_id: i32,
    pub progress_id: i32,
    pub type_id: i32,
}

#[derive(Args, Serialize, Deserialize)]
pub struct RequiredTaskFields {
    pub name: String,
    pub created_date: Option<i64>,
    pub started_date: Option<i64>,
    pub hours: Option<i32>,
    pub worked_hours: Option<i32>,
    pub status: bool,
    pub mode: bool,
    pub participant_id: i32,
    pub backlog_id: i32,
}

#[derive(Args, Serialize, Deserialize)]
pub struct RequiredCAFields {
    pub title: String,
    pub backlog_id: i32,
}
