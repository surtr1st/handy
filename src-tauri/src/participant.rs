use crate::establish_connection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct MentionOption {
    pub value: i32,
    pub label: String,
}

#[tauri::command]
pub fn get_participants() -> Vec<MentionOption> {
    use crate::schema::participants::dsl::{alias, id, participants};

    let connection = &mut establish_connection();

    participants
        .select((id, alias))
        .load::<MentionOption>(connection)
        .expect("all available participants should be returned!")
}
