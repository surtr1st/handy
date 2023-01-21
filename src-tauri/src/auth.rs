use super::establish_connection;
use super::models::User;
use super::schema::users::dsl::*;
use diesel::*;

#[tauri::command]
pub fn authenticate(_username: String, _password: String) -> Result<String, String> {
    let connection = &mut establish_connection();
    users
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .load::<User>(connection)
        .expect("Unauthorized");

    Ok("Login successfully!".into())
}

#[tauri::command]
pub fn registrate(_username: String, _password: String) -> Result<String, String> {
    let connection = &mut establish_connection();
    let new_user = (username.eq(_username), password.eq(_password));
    insert_into(users)
        .values(&new_user)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't Registrate!"));

    Ok("Registrated successfully!".into())
}
