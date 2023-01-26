use crate::args::RequiredTaskFields;
use crate::establish_connection;
use crate::models::Task;
use crate::ops::NewTask;
use diesel::prelude::*;

#[tauri::command]
pub fn get_tasks(_backlog_id: i32) -> Vec<Task> {
    use crate::schema::tasks;
    let connection = &mut establish_connection();
    tasks::table
        .filter(tasks::backlog_id.eq(_backlog_id))
        .load::<Task>(connection)
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
        pic: fields.pic,
        backlog_id: fields.backlog_id,
    };

    diesel::insert_into(tasks)
        .values(&task)
        .execute(connection)
        .expect("new task should be inserted!");

    Ok("Created task successfully!".into())
}

#[tauri::command]
pub fn update_task(fields: RequiredTaskFields) -> Result<String, String> {
    use crate::schema::tasks;
    let connection = &mut establish_connection();

    let new_data = Task {
        id: fields.id,
        name: fields.name,
        created_date: fields.created_date,
        started_date: fields.started_date,
        hours: fields.hours,
        worked_hours: fields.worked_hours,
        status: fields.status,
        mode: fields.mode,
        pic: fields.pic,
        backlog_id: fields.backlog_id,
    };

    diesel::update(tasks::table.find(fields.id))
        .set(&new_data)
        .execute(connection)
        .expect(&format!(
            "Task with id #{}, data entered should be updated!",
            fields.id
        ));

    Ok(format!("Updated task #{} successfully", fields.id).into())
}

#[tauri::command]
pub fn remove_task(task_id: i32) -> Result<String, String> {
    use crate::schema::tasks;
    let connection = &mut establish_connection();

    diesel::delete(tasks::table.find(task_id))
        .execute(connection)
        .expect(&format!("Task with id #{} should be removed!", task_id));

    Ok(format!("Removed Task #{} successfully!", task_id).into())
}
