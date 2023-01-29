#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod args;
pub mod auth;
pub mod backlog;
pub mod criteria_acceptance;
pub mod iteration;
pub mod models;
pub mod ops;
pub mod participant;
pub mod progress;
pub mod schema;
pub mod stats;
pub mod task;
pub mod worklog;

use auth::{authenticate, registrate};
use backlog::{create_backlog, get_backlog_types, get_backlogs, import_backlog};
use criteria_acceptance::{
    create_criteria_acceptance, get_criteria_acceptances, remove_criteria_acceptance,
    update_criteria_acceptance,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use iteration::{
    create_iteration, get_finished_iterations, get_iterations, get_joined_iterations,
    join_iteration,
};
use participant::{find_participant_alias, get_participants, get_personal_info};
use progress::get_progress_options;
use std::env;
use task::{create_task, get_tasks, remove_task, update_task};
use worklog::{get_worklogs, log_work};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            authenticate,
            registrate,
            get_participants,
            find_participant_alias,
            get_personal_info,
            get_iterations,
            get_joined_iterations,
            get_finished_iterations,
            create_iteration,
            join_iteration,
            create_backlog,
            import_backlog,
            get_backlogs,
            create_task,
            get_tasks,
            update_task,
            remove_task,
            get_criteria_acceptances,
            create_criteria_acceptance,
            update_criteria_acceptance,
            remove_criteria_acceptance,
            get_progress_options,
            get_backlog_types,
            get_worklogs,
            log_work
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
