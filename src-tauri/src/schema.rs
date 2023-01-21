// @generated automatically by Diesel CLI.

diesel::table! {
    backlog_types (id) {
        id -> Int4,
        name -> Text,
        backlog_id -> Nullable<Int4>,
    }
}

diesel::table! {
    backlogs (id) {
        id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        goals -> Text,
        priority -> Int4,
        hours -> Nullable<Int4>,
        points -> Nullable<Int4>,
        created_date -> Int8,
        iteration_id -> Int4,
        progress_id -> Int4,
        type_id -> Int4,
    }
}

diesel::table! {
    burndowns (id) {
        id -> Int4,
        ideal -> Nullable<Int4>,
        actual -> Nullable<Int4>,
        from_day -> Nullable<Int8>,
        iteration_id -> Int4,
    }
}

diesel::table! {
    criteria_acceptances (id) {
        id -> Int4,
        title -> Text,
        status -> Nullable<Bool>,
        backlog_id -> Int4,
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
    }
}

diesel::table! {
    participants (id) {
        id -> Text,
        name -> Text,
        backlog_id -> Int4,
        iteration_id -> Int4,
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
        hours -> Nullable<Int4>,
        worked_hours -> Nullable<Int4>,
        progress -> Text,
        mode -> Bool,
        pic -> Nullable<Text>,
        backlog_id -> Int4,
        progress_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    worklogs (id) {
        id -> Int4,
        description -> Text,
        total_hour -> Int4,
        task_id -> Int4,
        participant_id -> Nullable<Text>,
    }
}

diesel::joinable!(backlogs -> backlog_types (type_id));
diesel::joinable!(backlogs -> iterations (iteration_id));
diesel::joinable!(backlogs -> progresses (progress_id));
diesel::joinable!(burndowns -> iterations (iteration_id));
diesel::joinable!(criteria_acceptances -> backlogs (backlog_id));
diesel::joinable!(participants -> backlogs (backlog_id));
diesel::joinable!(participants -> iterations (iteration_id));
diesel::joinable!(tasks -> backlogs (backlog_id));
diesel::joinable!(tasks -> participants (pic));
diesel::joinable!(tasks -> progresses (progress_id));
diesel::joinable!(worklogs -> participants (participant_id));
diesel::joinable!(worklogs -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    backlog_types,
    backlogs,
    burndowns,
    criteria_acceptances,
    iterations,
    participants,
    progresses,
    tasks,
    users,
    worklogs,
);
