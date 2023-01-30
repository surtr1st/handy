use crate::establish_connection;
use crate::models::{IterationStatistic, ParticipantStatistic};
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
pub fn load_stats_of_iteration(_id: i32) -> IterationStatistic {
    use crate::schema::backlogs;
    use crate::schema::iterations;
    use crate::schema::progresses;

    let connection = &mut establish_connection();

    let backlog: i64 = backlogs::table
        .count()
        .inner_join(iterations::table)
        .filter(backlogs::iteration_id.eq(_id))
        .group_by(iterations::id)
        .get_result(connection)
        .unwrap_or(0);

    let backlog_done: i64 = backlogs::table
        .count()
        .inner_join(iterations::table)
        .inner_join(progresses::table)
        .filter(backlogs::iteration_id.eq(_id))
        .filter(progresses::name.eq("Done"))
        .group_by(backlogs::id)
        .get_result(connection)
        .unwrap_or(0);

    let backlog_partially_done: i64 = backlogs::table
        .count()
        .inner_join(iterations::table)
        .inner_join(progresses::table)
        .filter(backlogs::iteration_id.eq(_id))
        .filter(progresses::name.eq("Partially Done"))
        .group_by(backlogs::id)
        .get_result(connection)
        .unwrap_or(0);

    let backlog_undone: i64 = backlogs::table
        .count()
        .inner_join(iterations::table)
        .inner_join(progresses::table)
        .filter(backlogs::iteration_id.eq(_id))
        .filter(progresses::name.eq("Undone"))
        .group_by(backlogs::id)
        .get_result(connection)
        .unwrap_or(0);

    IterationStatistic {
        backlog,
        backlog_done,
        backlog_partially_done,
        backlog_undone,
        sprint_velocity: 0,
    }
}
