use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize)]
pub struct RequiredIterationFields {
    pub title: String,
    pub goals: String,
    pub created_by: String,
    pub created_date: i64,
    pub end_date: i64,
}
