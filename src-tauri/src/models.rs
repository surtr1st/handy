use crate::schema::tasks;
#[allow(unused)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct SelectOption {
    value: i32,
    label: String,
}

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct Personal {
    alias: String,
    password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct IterationKey {
    value: i32,
    label: String,
}

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct ReviewRetroIteration {
    start_date: i64,
    end_date: i64,
    total_point: i32,
}

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
    total_point: i32,
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
    current_hour: i32,
    points: i32,
    current_point: i32,
    created_date: i64,
    iteration_id: i32,
    progress_id: i32,
    type_id: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub created_date: Option<i64>,
    pub started_date: Option<i64>,
    pub hours: Option<i32>,
    pub worked_hours: Option<i32>,
    pub mode: bool,
    pub status: bool,
    pub participant_id: i32,
    pub backlog_id: i32,
}
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct TaskWithParticipantAlias {
    id: i32,
    name: String,
    created_date: Option<i64>,
    started_date: Option<i64>,
    hours: Option<i32>,
    worked_hours: Option<i32>,
    mode: bool,
    status: bool,
    participant_id: i32,
    backlog_id: i32,
    pic: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Worklog {
    id: i32,
    description: String,
    worked_hours: i32,
    task_id: i32,
    started_date: Option<i64>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct CriteriaAcceptance {
    pub id: i32,
    pub title: String,
    pub status: bool,
    pub backlog_id: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Burndown {
    id: i32,
    ideal: i32,
    actual: i32,
    from_day: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantStatistic {
    pub current: i64,
    pub attended: i64,
    pub finished: i64,
    pub created: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IterationStatistic {
    pub backlog: i64,
    pub backlog_done: i64,
    pub backlog_partially_done: i64,
    pub backlog_undone: i64,
    pub sprint_velocity: i64,
}
