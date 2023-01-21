use super::establish_connection;
use super::models::Iteration;
use super::schema::iterations::dsl::*;
use diesel::*;

#[tauri::command]
pub fn get_iterations() -> Vec<Iteration> {
    let connection = &mut establish_connection();
    iterations.load::<Iteration>(connection).unwrap()
}

#[tauri::command]
pub fn create_iteration(
    _title: String,
    _goals: String,
    _created_by: String,
    _created_date: i64,
    _end_date: i64,
) -> Result<String, String> {
    let connection = &mut establish_connection();

    if _title.is_empty() {
        return Err("Title is empty!".into());
    }
    if _goals.is_empty() {
        return Err("Goals is empty!".into());
    }
    if _created_by.is_empty() {
        return Err("Need at least one participant!".into());
    }
    if _created_date.is_negative() || _end_date.is_negative() {
        return Err("Timeline isn't set!".into());
    }

    let iteration = (
        title.eq(_title),
        goals.eq(_goals),
        current_point.eq(0),
        total_point.eq(0),
        created_by.eq(_created_by),
        created_date.eq(_created_date),
        end_date.eq(_end_date),
    );

    insert_into(iterations)
        .values(&iteration)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't create iteration!"));

    Ok("Created iteration successfully!".into())
}
