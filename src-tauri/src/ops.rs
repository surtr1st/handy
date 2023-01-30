use crate::schema::{backlogs, criteria_acceptances, iteration_rooms, iterations, tasks, worklogs};
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = iterations)]
pub struct NewIteration<'a> {
    pub title: &'a str,
    pub goals: &'a str,
    pub current_point: i32,
    pub total_point: i32,
    pub created_by: &'a str,
    pub created_date: i64,
    pub end_date: i64,
    pub status: bool,
}

#[derive(Insertable)]
#[diesel(table_name = backlogs)]
pub struct NewBacklog<'a> {
    pub title: &'a str,
    pub goals: &'a str,
    pub description: &'a str,
    pub priority: i32,
    pub hours: i32,
    pub current_hour: i32,
    pub points: i32,
    pub current_point: i32,
    pub created_date: i64,
    pub iteration_id: i32,
    pub progress_id: i32,
    pub type_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = iteration_rooms)]
pub struct NewIterationRoom {
    pub iteration_id: i32,
    pub participant_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub name: &'a str,
    pub created_date: Option<i64>,
    pub started_date: Option<i64>,
    pub hours: Option<i32>,
    pub worked_hours: Option<i32>,
    pub mode: bool,
    pub status: bool,
    pub participant_id: i32,
    pub backlog_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = criteria_acceptances)]
pub struct NewCriteriaAcceptance<'a> {
    pub title: &'a str,
    pub status: bool,
    pub backlog_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = worklogs)]
pub struct NewWorklog<'a> {
    pub description: &'a str,
    pub worked_hours: i32,
    pub task_id: i32,
    pub participant_id: i32,
}
