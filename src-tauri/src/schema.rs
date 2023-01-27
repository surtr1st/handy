// @generated automatically by Diesel CLI.

diesel::table! {
    backlog_types (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    backlogs (id) {
        id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        goals -> Text,
        priority -> Int4,
        hours -> Int4,
        points -> Int4,
        created_date -> Int8,
        iteration_id -> Int4,
        progress_id -> Int4,
        type_id -> Int4,
    }
}

diesel::table! {
    burndowns (id) {
        id -> Int4,
        ideal -> Int4,
        actual -> Int4,
        from_day -> Int8,
        iteration_id -> Int4,
    }
}

diesel::table! {
    completed_backlogs (id) {
        id -> Int4,
        completed_at -> Nullable<Int8>,
        backlog_id -> Int4,
    }
}

diesel::table! {
    completed_tasks (id) {
        id -> Int4,
        completed_at -> Nullable<Int8>,
        task_id -> Int4,
    }
}

diesel::table! {
    criteria_acceptances (id) {
        id -> Int4,
        title -> Text,
        status -> Bool,
        backlog_id -> Int4,
    }
}

diesel::table! {
    iteration_rooms (id) {
        id -> Int4,
        iteration_id -> Int4,
        participant_id -> Int4,
    }
}

diesel::table! {
    iterations (id) {
        id -> Int4,
        title -> Text,
        goals -> Text,
        current_point -> Nullable<Int4>,
        total_point -> Nullable<Int4>,
        created_by -> Text,
        created_date -> Int8,
        end_date -> Int8,
        status -> Nullable<Bool>,
    }
}

diesel::table! {
    participants (id) {
        id -> Int4,
        alias -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    progresses (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        name -> Text,
        created_date -> Nullable<Int8>,
        started_date -> Nullable<Int8>,
        hours -> Nullable<Int4>,
        worked_hours -> Nullable<Int4>,
        mode -> Bool,
        status -> Bool,
        pic -> Int4,
        backlog_id -> Int4,
    }
}

diesel::table! {
    worklogs (id) {
        id -> Int4,
        description -> Text,
        total_hour -> Int4,
        task_id -> Int4,
        participant_id -> Int4,
    }
}

diesel::joinable!(backlogs -> backlog_types (type_id));
diesel::joinable!(backlogs -> iterations (iteration_id));
diesel::joinable!(backlogs -> progresses (progress_id));
diesel::joinable!(burndowns -> iterations (iteration_id));
diesel::joinable!(completed_backlogs -> backlogs (backlog_id));
diesel::joinable!(completed_tasks -> tasks (task_id));
diesel::joinable!(criteria_acceptances -> backlogs (backlog_id));
diesel::joinable!(iteration_rooms -> iterations (iteration_id));
diesel::joinable!(iteration_rooms -> participants (participant_id));
diesel::joinable!(tasks -> backlogs (backlog_id));
diesel::joinable!(tasks -> participants (pic));
diesel::joinable!(worklogs -> participants (participant_id));
diesel::joinable!(worklogs -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    backlog_types,
    backlogs,
    burndowns,
    completed_backlogs,
    completed_tasks,
    criteria_acceptances,
    iteration_rooms,
    iterations,
    participants,
    progresses,
    tasks,
    worklogs,
);
