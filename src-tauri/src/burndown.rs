use crate::args::RequiredBurndownFields;
use crate::establish_connection;
use crate::ops::NewBurndown;
use diesel::prelude::*;

pub fn create_burndown(fields: RequiredBurndownFields) -> Result<(), String> {
    use crate::schema::burndowns::dsl::*;

    let connection = &mut establish_connection();
    let burndown = NewBurndown {
        ideal: fields.ideal,
        actual: fields.actual,
        from_day: &fields.from_day,
        iteration_id: fields.iteration_id,
    };

    diesel::insert_into(burndowns)
        .values(&(burndown))
        .execute(connection)
        .expect("new burndown should be inserted!");

    Ok(())
}

#[tauri::command]
pub fn update_burndown_actual_point(_id: i32) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_ideal_points(iteration_id: i32) -> Vec<i32> {
    use crate::schema::burndowns;
    use crate::schema::iterations;

    let connection = &mut establish_connection();
    burndowns::table
        .inner_join(iterations::table)
        .select(burndowns::ideal)
        .filter(burndowns::iteration_id.eq(iteration_id))
        .load::<i32>(connection)
        .expect(&format!(
            "ideal points of iteration with id #{} should have returned!",
            iteration_id
        ))
}

#[tauri::command]
pub fn get_total_point(iteration_id: i32) -> i32 {
    use crate::schema::burndowns;
    use crate::schema::iterations;

    let connection = &mut establish_connection();
    let total_point: i32 = burndowns::table
        .inner_join(iterations::table)
        .select(iterations::total_point)
        .filter(burndowns::iteration_id.eq(iteration_id))
        .get_result(connection)
        .expect(&format!(
            "total point of iteration with id #{} should have returned!",
            iteration_id
        ));
    total_point
}

#[tauri::command]
pub fn get_days(iteration_id: i32) -> Vec<String> {
    use crate::schema::burndowns;
    use crate::schema::iterations;

    let connection = &mut establish_connection();
    burndowns::table
        .inner_join(iterations::table)
        .select(burndowns::from_day)
        .filter(burndowns::iteration_id.eq(iteration_id))
        .load::<String>(connection)
        .expect(&format!(
            "burndown days from iteration #{} should have returned!",
            iteration_id
        ))
}

#[tauri::command]
pub fn get_current_points(iteration_id: i32) -> Vec<i32> {
    use crate::schema::burndowns;
    use crate::schema::iterations;

    let connection = &mut establish_connection();
    burndowns::table
        .inner_join(iterations::table)
        .select(burndowns::actual)
        .filter(burndowns::iteration_id.eq(iteration_id))
        .load::<i32>(connection)
        .expect(&format!(
            "burndown current points from iteration #{} should have returned!",
            iteration_id
        ))
}
