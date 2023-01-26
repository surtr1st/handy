use crate::args::RequiredBacklogFields;
use crate::establish_connection;
use crate::models::Backlog;
use crate::ops::NewBacklog;

use diesel::prelude::*;

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
        points: fields.points,
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
