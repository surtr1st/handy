use crate::args::RequiredTaskFields;
use crate::establish_connection;
use crate::models::{Task, TaskWithParticipantAlias};
use crate::ops::NewTask;
use diesel::prelude::*;

#[tauri::command]
pub fn get_tasks(_backlog_id: i32) -> Vec<TaskWithParticipantAlias> {
    use crate::schema::participants;
    use crate::schema::tasks;
    let connection = &mut establish_connection();
    tasks::table
        .inner_join(participants::table)
        .select((
            tasks::id,
            tasks::name,
            tasks::created_date,
            tasks::started_date,
            tasks::hours,
            tasks::worked_hours,
            tasks::mode,
            tasks::status,
            tasks::participant_id,
            tasks::backlog_id,
            participants::alias,
        ))
        .filter(tasks::backlog_id.eq(_backlog_id))
        .order_by(tasks::id.asc())
        .load::<TaskWithParticipantAlias>(connection)
        .expect("all task of backlog id should be returned!")
}

#[tauri::command]
pub fn create_task(fields: RequiredTaskFields) -> Result<String, String> {
    use crate::schema::tasks::dsl::*;
    let connection = &mut establish_connection();

    let task = NewTask {
        name: &fields.name,
        created_date: fields.created_date,
        started_date: fields.started_date,
        hours: fields.hours,
        worked_hours: fields.worked_hours,
        status: fields.status,
        mode: fields.mode,
        participant_id: fields.participant_id,
        backlog_id: fields.backlog_id,
    };

    diesel::insert_into(tasks)
        .values(&task)
        .execute(connection)
        .expect("new task should be inserted!");

    Ok("Created task successfully!".into())
}

#[tauri::command]
pub fn update_task(_id: i32, fields: RequiredTaskFields) -> Result<String, String> {
    use crate::schema::tasks;
    let connection = &mut establish_connection();

    let new_data = Task {
        id: _id,
        name: fields.name,
        created_date: fields.created_date,
        started_date: fields.started_date,
        hours: fields.hours,
        worked_hours: fields.worked_hours,
        status: fields.status,
        mode: fields.mode,
        participant_id: fields.participant_id,
        backlog_id: fields.backlog_id,
    };

    diesel::update(tasks::table.find(_id))
        .set(&new_data)
        .execute(connection)
        .expect(&format!(
            "Task with id #{}, data entered should be updated!",
            _id
        ));

    Ok(format!("Updated task #{} successfully", _id).into())
}

#[tauri::command]
pub fn remove_task(_id: i32) -> Result<String, String> {
    use crate::schema::tasks;
    let connection = &mut establish_connection();

    diesel::delete(tasks::table.find(_id))
        .execute(connection)
        .expect(&format!("Task with id #{} should be removed!", _id));

    Ok(format!("Removed Task #{} successfully!", _id).into())
}
