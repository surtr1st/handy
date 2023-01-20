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

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Participant {
    id: String,
    name: String,
}

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Iteration {
    id: i32,
    pub title: String,
    pub goals: String,
    pub current_point: Option<i32>,
    pub total_point: Option<i32>,
    pub created_by: String,
    pub created_date: i64,
    pub end_date: i64,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Backlog {
    id: i32,
    title: String,
    description: String,
    goals: String,
    priority: i32,
    hours: i32,
    points: i32,
    created_date: i64,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Task {
    id: i32,
    name: String,
    created_date: i64,
    hours: i32,
    worked_hours: i32,
    progress: String,
    mode: bool,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Worklog {
    id: i32,
    description: String,
    total_hour: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct CriteriaAcceptance {
    id: i32,
    title: String,
    status: bool,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Burndown {
    id: i32,
    ideal: i32,
    actual: i32,
    from_day: i64,
}
