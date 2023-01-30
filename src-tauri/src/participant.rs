use crate::establish_connection;
use crate::models::{Personal, SelectOption};
use diesel::prelude::*;

#[tauri::command]
pub fn get_participants() -> Vec<SelectOption> {
    use crate::schema::participants::dsl::{alias, id, participants};

    let connection = &mut establish_connection();
    participants
        .select((id, alias))
        .load::<SelectOption>(connection)
        .expect("all available participants should be returned!")
}

#[tauri::command]
pub fn find_participant_alias(_id: i32) -> Result<String, String> {
    use crate::schema::participants::dsl::{alias, id, participants};

    let connection = &mut establish_connection();
    let result = participants
        .select(alias)
        .filter(id.eq(_id))
        .load::<String>(connection)
        .expect("alias of participant should be returned!");

    let participant_alias = &result[0];
    Ok(participant_alias.into())
}

#[tauri::command]
pub fn get_personal_info(_id: i32) -> Result<Personal, String> {
    use crate::schema::participants::dsl::{alias, id, participants, password};

    let connection = &mut establish_connection();
    let result = participants
        .select((alias, password))
        .filter(id.eq(_id))
        .load::<Personal>(connection)
        .expect("personal info of participant should be returned!");

    let personal = result[0].clone();
    Ok(personal)
}

#[tauri::command]
pub fn update_alias(_id: i32) -> Result<String, String> {
    Ok("".into())
}

#[tauri::command]
pub fn update_password(_id: i32, _password: String) -> Result<String, String> {
    Ok("".into())
}
