use crate::{establish_connection, models::ParticipantStatistic};
use diesel::prelude::*;

#[tauri::command]
pub fn load_stats_of_participant(_id: i32) -> ParticipantStatistic {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();

    // total current iteration joining
    let current: i64 = iteration_rooms::table
        .count()
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .filter(iteration_rooms::participant_id.eq(_id))
        .filter(iterations::status.eq(false))
        .group_by(iteration_rooms::participant_id)
        .get_result(connection)
        .unwrap_or(0);

    // total iteration attended
    let attended: i64 = iteration_rooms::table
        .count()
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .filter(iteration_rooms::participant_id.eq(_id))
        .group_by(iteration_rooms::participant_id)
        .get_result(connection)
        .unwrap_or(0);

    // total finished iteration
    let finished: i64 = iteration_rooms::table
        .count()
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .filter(iteration_rooms::participant_id.eq(_id))
        .filter(iterations::status.eq(true))
        .group_by(iteration_rooms::id)
        .get_result(connection)
        .unwrap_or(0);

    // total iteration created
    let created_by: String = participants::table
        .select(participants::alias)
        .filter(participants::id.eq(_id))
        .get_result(connection)
        .expect(&format!(
            "participant alias with id #{} should be returned!",
            _id
        ));
    let created: i64 = iterations::table
        .count()
        .filter(iterations::created_by.eq(&created_by))
        .get_result(connection)
        .unwrap_or(0);

    ParticipantStatistic {
        current,
        attended,
        finished,
        created,
    }
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
