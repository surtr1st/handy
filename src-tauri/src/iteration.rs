use super::establish_connection;
use super::models::Iteration;
use super::schema::iterations::dsl::*;
use diesel::*;

#[tauri::command]
pub fn find_all() -> Vec<Iteration> {
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

    let iteration = (
        title.eq(_title),
        goals.eq(_goals),
        current_point.eq(0),
        total_point.eq(0),
        created_by.eq(_created_by),
        created_date.eq(_created_date),
        end_date.eq(_end_date),
    );

    let row_inserted = insert_into(iterations)
        .values(&iteration)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't create iteration!"));

    Ok("Created iteration successfully!").into()
}
