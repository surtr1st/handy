use crate::args::RequiredBacklogFields;
use crate::establish_connection;
use crate::models::{Backlog, SelectOption};
use crate::ops::NewBacklog;
use diesel::prelude::*;

#[tauri::command]
pub fn get_backlog_types() -> Vec<SelectOption> {
    use crate::schema::backlog_types::dsl::*;
    let connection = &mut establish_connection();
    backlog_types
        .select((id, name))
        .load::<SelectOption>(connection)
        .expect("all backlog types should be returned!")
}

#[tauri::command]
pub fn get_backlogs(_iteration_id: i32) -> Vec<Backlog> {
    use crate::schema::backlogs;
    let connection = &mut establish_connection();
    backlogs::table
        .filter(backlogs::iteration_id.eq(_iteration_id))
        .load::<Backlog>(connection)
        .expect("all backlog should be returned within iteration by id")
}

#[tauri::command]
pub fn create_backlog(fields: RequiredBacklogFields) -> Result<String, String> {
    use crate::schema::backlogs::dsl::*;

    let connection = &mut establish_connection();

    let backlog = NewBacklog {
        title: &fields.title,
        goals: &fields.goals,
        description: &fields.description,
        priority: fields.priority,
        hours: fields.hours,
        current_hour: fields.current_hour,
        points: fields.points,
        current_point: fields.current_point,
        created_date: fields.created_date,
        iteration_id: fields.iteration_id,
        progress_id: fields.progress_id,
        type_id: fields.type_id,
    };

    diesel::insert_into(backlogs)
        .values(&backlog)
        .execute(connection)
        .expect("new backlog should be inserted!");

    Ok("Created backlog successfully".into())
}

#[tauri::command]
pub fn update_backlog_hours(_id: i32, _hours: i32) -> Result<(), String> {
    use crate::schema::backlogs::dsl::*;

    let connection = &mut establish_connection();
    diesel::update(backlogs.find(_id))
        .set(hours.eq(_hours))
        .execute(connection)
        .expect(&format!(
            "hours of backlog with id #{} should be updated!",
            _id
        ));
    Ok(())
}

#[tauri::command]
pub fn update_backlog_current_hour(_id: i32, current: i32) -> Result<(), String> {
    use crate::schema::backlogs::dsl::*;

    let connection = &mut establish_connection();
    diesel::update(backlogs.find(_id))
        .set(current_hour.eq(current))
        .execute(connection)
        .expect(&format!(
            "current hour of backlog with id #{} should be updated!",
            _id
        ));
    Ok(())
}

#[tauri::command]
pub fn update_backlog_points(_id: i32, _point: i32) -> Result<(), String> {
    use crate::schema::backlogs::dsl::*;

    let connection = &mut establish_connection();
    diesel::update(backlogs.find(_id))
        .set(points.eq(_point))
        .execute(connection)
        .expect(&format!(
            "points of backlog with id #{} should be updated!",
            _id
        ));
    Ok(())
}

#[tauri::command]
pub fn update_backlog_current_point(_id: i32, current: i32) -> Result<(), String> {
    use crate::schema::backlogs::dsl::*;

    let connection = &mut establish_connection();
    diesel::update(backlogs.find(_id))
        .set(current_point.eq(current))
        .execute(connection)
        .expect(&format!(
            "current point of backlog with id #{} should be updated!",
            _id
        ));
    Ok(())
}

fn get_first_arg() -> Result<std::ffi::OsString, Box<dyn std::error::Error>> {
    match std::env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[tauri::command]
pub fn import_backlog() -> Result<String, String> {
    let filepath = get_first_arg().unwrap();
    let file = std::fs::File::open(filepath).unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let record: RequiredBacklogFields = result.expect("each row should be listed!");
        println!("{:?}", record);
    }
    Ok("Imported backlogs successfully!".into())
}
