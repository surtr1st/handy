use super::establish_connection;
use super::schema::participants::dsl::*;
use diesel::*;

#[tauri::command]
pub fn authenticate(_username: String, _password: String) -> Result<(String, String), String> {
    let connection = &mut establish_connection();

    if _username.is_empty() {
        return Err("Incorrect username!".into());
    }
    if _password.is_empty() {
        return Err("Incorrect password!".into());
    }

    let participant_id = participants
        .select(id)
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .load::<i32>(connection)
        .unwrap_or_else(|_| panic!("Couldn't authenticate!"));
    if participant_id.is_empty() {
        return Err("Wrong username or password!".into());
    }

    Ok((
        "Login successfully!".into(),
        participant_id[0].to_string().into(),
    ))
}

#[tauri::command]
pub fn registrate(_username: String, _password: String) -> Result<String, String> {
    let connection = &mut establish_connection();

    if _username.is_empty() {
        return Err("Username is empty!".into());
    }
    if _username.len() < 4 {
        return Err("Username should be at least 4 characters!".into());
    }
    if _password.is_empty() {
        return Err("Password is empty!".into());
    }
    if _password.len() < 3 {
        return Err("Password should be at least 3 characters!".into());
    }

    let new_user = (
        alias.eq(format!("@{}", _username)),
        username.eq(_username),
        password.eq(_password),
    );
    insert_into(participants)
        .values(&new_user)
        .execute(connection)
        .unwrap_or_else(|_| panic!("Couldn't Registrate!"));

    Ok("Registrated successfully!".into())
}
