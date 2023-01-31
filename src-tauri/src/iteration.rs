use crate::args::{RequiredBurndownFields, RequiredIterationFields};
use crate::burndown::create_burndown;
use crate::establish_connection;
use crate::models::{Iteration, IterationKey, ReviewRetroIteration};
use crate::ops::{NewIteration, NewIterationRoom};
use chrono::{Duration, NaiveDate}; // 0.4.23
use diesel::prelude::*;

#[tauri::command]
pub fn get_iteration_keys(_participant_id: i32) -> Vec<IterationKey> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();
    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select((iterations::id, iterations::title))
        .filter(iteration_rooms::participant_id.eq(_participant_id))
        .group_by(iterations::id)
        .load::<IterationKey>(connection)
        .expect("all joined iteration should be returned!")
}

#[tauri::command]
pub fn get_iterations(_iteration_id: i32, _participant_id: i32) -> Vec<Iteration> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();
    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select(iterations::all_columns)
        .filter(iteration_rooms::iteration_id.ne(_iteration_id))
        .filter(iteration_rooms::participant_id.ne(_participant_id))
        .group_by(iterations::id)
        .load::<Iteration>(connection)
        .expect("all iteration beside joined iterations should be returned!")
}

#[tauri::command]
pub fn get_iteration_data_when_review_retro(_id: i32) -> ReviewRetroIteration {
    use crate::schema::iterations::dsl::*;

    let connection = &mut establish_connection();
    iterations
        .select((created_date, end_date, total_point))
        .filter(id.eq(_id))
        .get_result(connection)
        .expect(&format!("iteration with id #{} should be returned!", _id))
}

#[tauri::command]
pub fn get_joined_iterations(_participant_id: i32) -> Vec<Iteration> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();
    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select(iterations::all_columns)
        .filter(iteration_rooms::participant_id.eq(_participant_id))
        .filter(iterations::status.eq(false))
        .group_by(iterations::id)
        .load::<Iteration>(connection)
        .expect("all joined iteration should be returned!")
}

#[tauri::command]
pub fn get_finished_iterations(_participant_id: i32) -> Vec<Iteration> {
    use crate::schema::iteration_rooms;
    use crate::schema::iterations;
    use crate::schema::participants;

    let connection = &mut establish_connection();
    iteration_rooms::table
        .inner_join(iterations::table)
        .inner_join(participants::table)
        .select(iterations::all_columns)
        .filter(iteration_rooms::participant_id.eq(_participant_id))
        .filter(iterations::status.eq(true))
        .group_by(iterations::id)
        .load::<Iteration>(connection)
        .expect("all iteration which user finished should be returned!")
}

#[tauri::command]
pub fn create_iteration(fields: RequiredIterationFields) -> Result<String, String> {
    use crate::schema::iterations::dsl::*;
    use crate::schema::participants::dsl::{alias, id as pid, participants};

    let connection = &mut establish_connection();

    if fields.title.is_empty() {
        return Err("Title is empty!".into());
    }
    if fields.goals.is_empty() {
        return Err("Goals is empty!".into());
    }
    if fields.created_date.is_negative() || fields.end_date.is_negative() {
        return Err("Timeline isn't set!".into());
    }

    let owner: String = participants
        .select(alias)
        .filter(pid.eq(fields.created_by))
        .get_result(connection)
        .expect("alias owner of chosen iteration should be returned!");

    let iteration = NewIteration {
        title: &fields.title,
        goals: &fields.goals,
        current_point: 0,
        total_point: fields.points,
        created_by: &owner,
        created_date: fields.created_date,
        end_date: fields.end_date,
        status: false,
    };

    diesel::insert_into(iterations)
        .values(&iteration)
        .execute(connection)
        .expect("new iteration value should be inserted!");

    let latest_iteration_id_created: i32 = iterations
        .select(id)
        .filter(created_by.eq(&owner))
        .order_by(id.desc())
        .limit(1)
        .get_result(connection)
        .expect("latest iteration inserted should be returned!");

    let latest_iteration_point_created: i32 = iterations
        .select(total_point)
        .filter(created_by.eq(&owner))
        .order_by(id.desc())
        .limit(1)
        .get_result(connection)
        .expect("latest iteration point inserted should be returned!");

    create_iteration_room(latest_iteration_id_created, fields.participants);
    make_burndown(
        latest_iteration_id_created,
        fields.start_day,
        fields.end_day,
        latest_iteration_point_created,
    );

    Ok("Created iteration successfully!".into())
}

fn days_between_dates(_start: String, _end: String) -> i32 {
    let start = from_1971_01_01(_start);
    let end = from_1971_01_01(_end);
    (start - end).abs()
}

fn from_1971_01_01(date: String) -> i32 {
    let m_d = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let date = &date
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut days = date[2] - 1 + m_d[date[1] as usize - 1];
    if date[0] % 4 == 0 && date[0] != 2100 && date[1] > 2 {
        days += 1;
    }
    days += 365 * (date[0] - 1971) + (date[0] - 1969) / 4;
    days
}

fn sum_array_up_to(arr: Vec<i32>, index: i32) -> i32 {
    let mut total = 0;
    for i in 0..=index {
        if arr.len() > i as usize {
            total += arr[i as usize];
        }
    }
    total
}

fn calculate_ideal_point(days: i32, point: i32, scope_change: Vec<i32>, index: i32) -> i32 {
    let total_hours_in_sprint = point;
    let ideal_hours_per_day = total_hours_in_sprint / days;
    total_hours_in_sprint - ideal_hours_per_day * (index + 1) + sum_array_up_to(scope_change, index)
}

fn make_burndown(iteration_id: i32, start: String, end: String, points: i32) {
    let days = days_between_dates(start.clone(), end.clone()) + 1;

    create_burndown(RequiredBurndownFields {
        ideal: points,
        actual: points,
        from_day: start.clone(),
        iteration_id,
    })
    .unwrap();

    for day in 1..days {
        let date =
            NaiveDate::parse_from_str(&start, "%Y-%m-%d").unwrap() + Duration::days(day as i64);
        let ideal = calculate_ideal_point(days, points, vec![0; days as usize], day);
        create_burndown(RequiredBurndownFields {
            ideal,
            actual: 0,
            from_day: date.to_string(),
            iteration_id,
        })
        .unwrap();
    }
}

#[tauri::command]
pub fn join_iteration(iteration_id: i32, participant_id: i32) -> Result<String, String> {
    create_iteration_room(iteration_id, vec![participant_id]);
    Ok(format!("Joined Iteration #{}", iteration_id).into())
}

fn create_iteration_room(iteration_id: i32, participants: Vec<i32>) {
    use crate::schema::iteration_rooms;

    let connection = &mut establish_connection();
    for participant in &participants {
        let iteration_room = NewIterationRoom {
            iteration_id,
            participant_id: *participant,
        };
        diesel::insert_into(iteration_rooms::table)
            .values(&iteration_room)
            .execute(connection)
            .expect("participant should be joined!");
    }
}

#[tauri::command]
pub fn end_iteration(
    _iteration_id: i32,
    _review_content: String,
    _retro_content: String,
) -> Result<String, String> {
    use crate::schema::iterations::dsl::{iterations, status};

    let connection = &mut establish_connection();
    diesel::update(iterations.find(_iteration_id))
        .set(status.eq(true))
        .execute(connection)
        .expect(&format!(
            "Iteration with id #{} should be ended!",
            _iteration_id
        ));
    // read content
    // write file
    // save

    Ok(format!("Successfully ended iteration #{}!", _iteration_id).into())
}
