use crate::schema::{backlogs, iteration_rooms, iterations};
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
    pub points: i32,
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
