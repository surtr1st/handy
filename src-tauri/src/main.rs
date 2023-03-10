#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod args;
pub mod auth;
pub mod backlog;
pub mod burndown;
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
use backlog::{
    create_backlog, get_backlog_types, get_backlogs, import_backlog, update_backlog_current_hour,
    update_backlog_current_point, update_backlog_hours, update_backlog_points,
};
use burndown::{
    get_current_points, get_days, get_ideal_points, get_total_point, update_burndown_actual_point,
};
use criteria_acceptance::{
    create_criteria_acceptance, get_criteria_acceptances, remove_criteria_acceptance,
    update_criteria_acceptance,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use iteration::{
    create_iteration, end_iteration, get_finished_iterations, get_iteration_data_when_review_retro,
    get_iteration_keys, get_iterations, get_joined_iterations, join_iteration,
};
use participant::{find_participant_alias, get_participants, get_personal_info};
use progress::get_progress_options;
use stats::{load_stats_of_iteration, load_stats_of_participant};
use std::env;
use task::{
    create_task, get_tasks, get_tasks_done, remove_task, update_task_after_logwork,
    update_task_hours, update_task_name, update_task_started_date,
};
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
            // Participant
            authenticate,
            registrate,
            get_participants,
            get_personal_info,
            find_participant_alias,
            // Iteration
            get_iterations,
            get_joined_iterations,
            get_finished_iterations,
            get_iteration_keys,
            get_iteration_data_when_review_retro,
            create_iteration,
            join_iteration,
            end_iteration,
            // Backlog
            get_backlog_types,
            get_backlogs,
            create_backlog,
            import_backlog,
            update_backlog_hours,
            update_backlog_points,
            update_backlog_current_hour,
            update_backlog_current_point,
            // Task
            get_tasks,
            get_tasks_done,
            create_task,
            update_task_name,
            update_task_started_date,
            update_task_hours,
            update_task_after_logwork,
            remove_task,
            // Criteria Acceptance
            get_criteria_acceptances,
            create_criteria_acceptance,
            update_criteria_acceptance,
            remove_criteria_acceptance,
            // Progress
            get_progress_options,
            // Worklog
            get_worklogs,
            log_work,
            // Synthetic Statistic
            load_stats_of_participant,
            load_stats_of_iteration,
            // Burndown
            update_burndown_actual_point,
            get_days,
            get_current_points,
            get_total_point,
            get_ideal_points
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
