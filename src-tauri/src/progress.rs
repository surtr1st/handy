use crate::establish_connection;
use crate::models::SelectOption;
use diesel::prelude::*;

#[tauri::command]
pub fn get_progress_options() -> Vec<SelectOption> {
    use crate::schema::progresses::dsl::*;
    let connection = &mut establish_connection();
    progresses
        .select((id, name))
        .load::<SelectOption>(connection)
        .expect("all progress type should be returned!")
}
