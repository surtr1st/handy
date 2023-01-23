use crate::args::RequiredIterationFields;
use crate::establish_connection;
use crate::models::{Iteration, NewIteration};

use diesel::prelude::*;

#[tauri::command]
pub fn get_iterations(_participant_id: i32) -> Vec<Iteration> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;
    let connection = &mut establish_connection();

    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select(iterations::all_columns)
        .filter(iteration_rooms::participant_id.eq(_participant_id))
        .group_by(iterations::id)
        .load::<Iteration>(connection)
        .unwrap()
}

#[tauri::command]
pub fn get_finished_iterations(_participant_id: i32) -> Vec<Iteration> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;
    let connection = &mut establish_connection();

    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select(iterations::all_columns)
        .filter(iteration_rooms::participant_id.eq(_participant_id))
        .filter(iterations::status.eq(true))
        .group_by(iterations::id)
        .load::<Iteration>(connection)
        .unwrap()
}

#[tauri::command]
pub fn create_iteration(fields: RequiredIterationFields) -> Result<String, String> {
    use crate::schema::iterations::dsl::*;
    let connection = &mut establish_connection();

    if fields.title.is_empty() {
        return Err("Title is empty!".into());
    }
    if fields.goals.is_empty() {
        return Err("Goals is empty!".into());
    }
    if fields.created_by.is_empty() {
        return Err("Need at least one participant!".into());
    }
    if fields.created_date.is_negative() || fields.end_date.is_negative() {
        return Err("Timeline isn't set!".into());
    }

    let iteration = NewIteration {
        title: &fields.title,
        goals: &fields.goals,
        current_point: 0,
        total_point: 0,
        created_by: &fields.created_by,
        created_date: fields.created_date,
        end_date: fields.end_date,
        status: false,
    };

    diesel::insert_into(iterations)
        .values(&iteration)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't create iteration!"));

    Ok("Created iteration successfully!".into())
}

#[tauri::command]
pub fn join_iteration(_iteration_id: i32, _participant_id: i32) -> Result<String, String> {
    use crate::schema::iteration_rooms::dsl::*;
    let connection = &mut establish_connection();

    let room = (
        iteration_id.eq(_iteration_id),
        participant_id.eq(_participant_id),
    );

    diesel::insert_into(iteration_rooms)
        .values(&room)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't join iteration!"));

    Ok(format!("Joined Iteration #{}", _iteration_id).into())
}

pub fn end_iteration(_iteration_id: i32) {}
