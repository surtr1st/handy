use crate::establish_connection;
use crate::models::SelectOption;
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
