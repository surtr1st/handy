#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod args;
pub mod auth;
pub mod backlog;
pub mod iteration;
pub mod models;
pub mod ops;
pub mod participant;
pub mod schema;

use auth::{authenticate, registrate};
use backlog::{create_backlog, get_backlogs, import_backlog};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use iteration::{
    create_iteration, get_finished_iterations, get_iterations, get_joined_iterations,
    join_iteration,
};
use participant::get_participants;
use std::env;

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
            get_iterations,
            get_joined_iterations,
            get_finished_iterations,
            create_iteration,
            join_iteration,
            create_backlog,
            import_backlog,
            get_backlogs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
