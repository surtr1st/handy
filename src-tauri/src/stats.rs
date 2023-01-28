use diesel::prelude::*;

#[tauri::command]
pub fn load_stats_of_participant(_id: i32) -> Result<(), String> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;
    Ok(())
}

#[tauri::command]
pub fn load_stats_of_iteration(_id: i32) -> Result<(), String> {
    use crate::schema::iterations;
    Ok(())
}

#[tauri::command]
pub fn load_stats_of_backlog(_id: i32) -> Result<(), String> {
    use crate::schema::backlogs;
    Ok(())
}
