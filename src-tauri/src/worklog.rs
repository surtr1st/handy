use crate::args::RequiredWorklogFields;
use crate::establish_connection;
use crate::models::Worklog;
use crate::ops::NewWorklog;
use diesel::prelude::*;

#[tauri::command]
pub fn get_worklogs(_participant_id: i32) -> Vec<Worklog> {
    use crate::schema::tasks;
    use crate::schema::worklogs;
    let connection = &mut establish_connection();
    worklogs::table
        .inner_join(tasks::table)
        .select((
            worklogs::id,
            worklogs::description,
            worklogs::worked_hours,
            tasks::id,
            tasks::started_date,
        ))
        .filter(worklogs::participant_id.eq(_participant_id))
        .load::<Worklog>(connection)
        .expect("all worklog of participant id should be returned!")
}

#[tauri::command]
pub fn log_work(fields: RequiredWorklogFields) -> Result<String, String> {
    use crate::schema::worklogs::dsl::*;
    let connection = &mut establish_connection();
    let worklog = NewWorklog {
        description: &fields.description,
        worked_hours: fields.worked_hours,
        task_id: fields.task_id,
        participant_id: fields.participant_id,
    };

    diesel::insert_into(worklogs)
        .values(&worklog)
        .execute(connection)
        .expect("new worklog should be inserted!");

    Ok(format!("Logworked task #{}", fields.task_id).into())
}
