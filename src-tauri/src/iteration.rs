use crate::args::RequiredIterationFields;
use crate::establish_connection;
use crate::models::{Iteration, IterationKey, ReviewRetroIteration};
use crate::ops::{NewIteration, NewIterationRoom};

use diesel::prelude::*;

#[tauri::command]
pub fn get_iteration_keys(_participant_id: i32) -> Vec<IterationKey> {
use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();
    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select((iterations::id, iterations::title))
        .filter(iteration_rooms::participant_id.eq(_participant_id))
        .group_by(iterations::id)
        .load::<IterationKey>(connection)
        .expect("all joined iteration should be returned!")
}

#[tauri::command]
pub fn get_iterations(_iteration_id: i32, _participant_id: i32) -> Vec<Iteration> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();
    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select(iterations::all_columns)
        .filter(iteration_rooms::iteration_id.ne(_iteration_id))
        .filter(iteration_rooms::participant_id.ne(_participant_id))
        .group_by(iterations::id)
        .load::<Iteration>(connection)
        .expect("all iteration beside joined iterations should be returned!")
}

#[tauri::command]
pub fn get_iteration_data_when_review_retro(_id: i32) -> ReviewRetroIteration {
    use crate::schema::iterations::dsl::*;

    let connection = &mut establish_connection();
    let result = iterations
        .select((created_date, end_date, total_point))
        .filter(id.eq(_id))
        .load::<ReviewRetroIteration>(connection)
        .expect(&format!("iteration with id #{} should be returned!", _id));

    result[0].clone()
}

#[tauri::command]
pub fn get_joined_iterations(_participant_id: i32) -> Vec<Iteration> {
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
        .expect("all joined iteration should be returned!")
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
        .expect("all iteration which user finished should be returned!")
}

#[tauri::command]
pub fn create_iteration(fields: RequiredIterationFields) -> Result<String, String> {
    use crate::schema::iterations::dsl::*;
    use crate::schema::participants::dsl::{alias, id as pid, participants};

    let connection = &mut establish_connection();

    if fields.title.is_empty() {
        return Err("Title is empty!".into());
    }
    if fields.goals.is_empty() {
        return Err("Goals is empty!".into());
    }
    if fields.created_date.is_negative() || fields.end_date.is_negative() {
        return Err("Timeline isn't set!".into());
    }

    let owner = participants
        .select(alias)
        .filter(pid.eq(fields.created_by))
        .load::<String>(connection)
        .expect("alias owner of chosen iteration should be returned!");

    let iteration = NewIteration {
        title: &fields.title,
        goals: &fields.goals,
        current_point: 0,
        total_point: 0,
        created_by: &owner[0],
        created_date: fields.created_date,
        end_date: fields.end_date,
        status: false,
    };

    diesel::insert_into(iterations)
        .values(&iteration)
        .execute(connection)
        .expect("new iteration value should be inserted!");

    let latest_iteration_id_created = iterations
        .select(id)
        .filter(created_by.eq(&owner[0]))
        .order_by(id.desc())
        .limit(1)
        .load::<i32>(connection)
        .expect("latest iteration inserted should be returned!");

    create_iteration_room(latest_iteration_id_created[0], fields.participants);

    Ok("Created iteration successfully!".into())
}

#[tauri::command]
pub fn join_iteration(iteration_id: i32, participant_id: i32) -> Result<String, String> {
    create_iteration_room(iteration_id, vec![participant_id]);
    Ok(format!("Joined Iteration #{}", iteration_id).into())
}

fn create_iteration_room(iteration_id: i32, participants: Vec<i32>) {
    use crate::schema::iteration_rooms;

    let connection = &mut establish_connection();
    for participant in &participants {
        let iteration_room = NewIterationRoom {
            iteration_id,
            participant_id: *participant,
        };
        diesel::insert_into(iteration_rooms::table)
            .values(&iteration_room)
            .execute(connection)
            .expect("participant should be joined!");
    }
}

#[tauri::command]
pub fn end_iteration(
    _iteration_id: i32,
    review_content: String,
    retro_content: String,
) -> Result<String, String> {
    use crate::schema::iterations::dsl::{iterations, status};

    let connection = &mut establish_connection();
    diesel::update(iterations)
        .set(status.eq(true))
        .execute(connection)
        .expect(&format!(
            "Iteration with id #{} should be ended!",
            _iteration_id
        ));
    // read content
    // write file
    // save

    Ok("Successfully ended iteration!".into())
}
