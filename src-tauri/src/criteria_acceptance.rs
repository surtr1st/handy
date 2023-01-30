use crate::args::RequiredCAFields;
use crate::establish_connection;
use crate::models::CriteriaAcceptance;
use crate::ops::NewCriteriaAcceptance;
use diesel::prelude::*;

#[tauri::command]
pub fn get_criteria_acceptances(_backlog_id: i32) -> Vec<CriteriaAcceptance> {
    use crate::schema::criteria_acceptances::dsl::*;
    let connection = &mut establish_connection();
    criteria_acceptances
        .filter(backlog_id.eq(_backlog_id))
        .load::<CriteriaAcceptance>(connection)
        .expect(&format!(
            "all CA from backlog #{} should be returned!",
            _backlog_id
        ))
}

#[tauri::command]
pub fn create_criteria_acceptance(fields: RequiredCAFields) -> Result<String, String> {
    use crate::schema::criteria_acceptances::dsl::*;
    let connection = &mut establish_connection();

    let ca = NewCriteriaAcceptance {
        title: &fields.title,
        status: false,
        backlog_id: fields.backlog_id,
    };

    diesel::insert_into(criteria_acceptances)
        .values(&ca)
        .execute(connection)
        .expect("New criteria acceptance should be inserted!");

    Ok("Added 1 criteria acceptance!".into())
}

#[tauri::command]
pub fn update_criteria_acceptance(_id: i32, _backlog_id: i32, value: bool) -> Result<(), String> {
    use crate::schema::criteria_acceptances::dsl::*;
    let connection = &mut establish_connection();

    diesel::update(
        criteria_acceptances
            .filter(id.eq(_id))
            .filter(backlog_id.eq(_backlog_id)),
    )
    .set(status.eq(value))
    .execute(connection)
    .expect(&format!("CA with id #{} should be updated!", _id));

    Ok(())
}

#[tauri::command]
pub fn remove_criteria_acceptance(_id: i32, _backlog_id: i32) -> Result<(), String> {
    use crate::schema::criteria_acceptances::dsl::*;
    let connection = &mut establish_connection();

    diesel::delete(
        criteria_acceptances
            .filter(id.eq(_id))
            .filter(backlog_id.eq(_backlog_id)),
    )
    .execute(connection)
    .expect(&format!("CA with id #{} should be removed!", _id));

    Ok(())
}
