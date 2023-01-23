use super::establish_connection;
use super::models::Iteration;
use super::schema::iteration_rooms::dsl::*;
use super::schema::iterations::dsl::*;
use super::schema::participants::dsl::*;

use diesel::*;

#[tauri::command]
pub fn get_iterations() -> Vec<Iteration> {
    let connection = &mut establish_connection();
    iterations.load::<Iteration>(connection).unwrap()
    // filter iterations which participant haven't joined
    //     iteration_rooms
    //         .inner_join(iterations)
    //         .inner_join(participants)
    //         .select((
    //             iid,
    //             title,
    //             current_point,
    //             total_point,
    //             created_date,
    //             end_date,
    //         ))
    //         .group_by(iid)
    //         .load(connection)
    //         .unwrap()
}

#[derive(Clone)]
struct JoinedIteration {
    id: i32,
    title: String,
    current_point: i32,
    total_point: i32,
    created_date: i64,
    end_date: i64,
}

#[tauri::command]
pub fn get_joined_iterations(_participant_id: i32) -> Vec<Iteration> {
    let connection = &mut establish_connection();
    iteration_rooms
        .inner_join(iterations)
        .inner_join(participants)
        .filter(participant_id.eq(_participant_id))
        .group_by(super::schema::iterations::id)
        .load::<Iteration>(connection)
        .unwrap()
}

#[tauri::command]
pub fn create_iteration(
    _title: String,
    _goals: String,
    _created_by: String,
    _created_date: i64,
    _end_date: i64,
) -> Result<String, String> {
    let connection = &mut establish_connection();

    if _title.is_empty() {
        return Err("Title is empty!".into());
    }
    if _goals.is_empty() {
        return Err("Goals is empty!".into());
    }
    if _created_by.is_empty() {
        return Err("Need at least one participant!".into());
    }
    if _created_date.is_negative() || _end_date.is_negative() {
        return Err("Timeline isn't set!".into());
    }

    let iteration = (
        title.eq(_title),
        goals.eq(_goals),
        current_point.eq(0),
        total_point.eq(0),
        created_by.eq(_created_by),
        created_date.eq(_created_date),
        end_date.eq(_end_date),
    );

    insert_into(iterations)
        .values(&iteration)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't create iteration!"));

    Ok("Created iteration successfully!".into())
}

#[tauri::command]
pub fn join_iteration(_iteration_id: i32, _participant_id: i32) -> Result<String, String> {
    let connection = &mut establish_connection();

    let room = (
        iteration_id.eq(_iteration_id),
        participant_id.eq(_participant_id),
    );

    insert_into(iteration_rooms)
        .values(&room)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't join iteration!"));

    Ok(format!("Joined Iteration #{}", _iteration_id).into())
}

pub fn end_iteration(_iteration_id: i32) {}
