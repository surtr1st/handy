use crate::schema::tasks;
#[allow(unused)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct BacklogType {
    id: i32,
    name: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Progress {
    id: i32,
    name: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Participant {
    id: i32,
    alias: String,
    username: String,
    password: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Iteration {
    id: i32,
    title: String,
    goals: String,
    current_point: Option<i32>,
    total_point: Option<i32>,
    created_by: String,
    created_date: i64,
    end_date: i64,
    status: Option<bool>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct IterationRoom {
    id: i32,
    iteration_id: i32,
    participant_id: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Backlog {
    id: i32,
    title: String,
    description: Option<String>,
    goals: String,
    priority: i32,
    hours: i32,
    points: i32,
    created_date: i64,
    iteration_id: i32,
    progress_id: i32,
    type_id: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct Task {
    id: i32,
    name: String,
    created_date: Option<i64>,
    started_date: Option<i64>,
    hours: Option<i32>,
    worked_hours: Option<i32>,
    mode: bool,
    status: bool,
    pic: i32,
    backlog_id: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Worklog {
    id: i32,
    description: String,
    total_hour: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct CriteriaAcceptance {
    id: i32,
    title: String,
    status: bool,
    backlog_id: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Burndown {
    id: i32,
    ideal: i32,
    actual: i32,
    from_day: i64,
}
