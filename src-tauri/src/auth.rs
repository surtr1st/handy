use super::establish_connection;
use super::models::User;
use super::schema::users::dsl::*;
use diesel::*;

#[tauri::command]
pub fn authenticate(_username: &str, _password: &str) -> String {
    let connection = &mut establish_connection();
    let results = users
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .load::<User>(connection)
        .expect("Unauthorized");

    format!("{:?}", results)
}
