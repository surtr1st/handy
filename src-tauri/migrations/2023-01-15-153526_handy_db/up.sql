-- Your SQL goes here

CREATE TABLE
  progresses (
    id serial primary key not null,
    name text not null
  );

CREATE TABLE
  iterations (
    id serial primary key not null,
    title text not null,
    goals text not null,
    current_point int,
    total_point int,
    created_by text not null,
    created_date bigint not null,
    end_date bigint not null,
    status boolean
  );

CREATE TABLE
  backlog_types (
    id serial primary key not null,
    name text not null
  );

CREATE TABLE
  backlogs (
    id serial primary key not null,
    title text not null,
    description text,
    goals text not null,
    priority int not null,
    hours int not null,
    current_hour int not null,
    points int not null,
    current_point int not null,
    created_date bigint not null,
    iteration_id serial references iterations (id),
    progress_id serial references progresses (id),
    type_id serial references backlog_types (id)
  );

CREATE TABLE
  participants (
    id serial primary key not null,
    alias text not null,
    username text not null,
    password text not null
  );

CREATE TABLE
  tasks (
    id serial primary key not null,
    name text not null,
    created_date bigint,
    started_date bigint,
    hours int,
    worked_hours int,
    mode boolean not null,
    status boolean not null,
    participant_id serial references participants (id),
    backlog_id serial references backlogs (id)
  );

CREATE TABLE
  worklogs (
    id serial primary key not null,
    description text not null,
    worked_hours int not null,
    task_id serial references tasks (id),
    participant_id serial references participants (id)
  );

CREATE TABLE
  criteria_acceptances (
    id serial primary key not null,
    title text not null,
    status boolean not null,
    backlog_id serial references backlogs (id)
  );

CREATE TABLE
  burndowns (
    id serial primary key not null,
    ideal int not null,
    actual int not null,
    from_day bigint,
    iteration_id serial references iterations (id)
  );

CREATE TABLE
  iteration_rooms (
    id serial primary key not null,
    iteration_id serial references iterations (id),
    participant_id serial references participants (id)
  );

CREATE TABLE
  partial_done_backlogs (
    id serial primary key not null,
    backlog_id serial references backlogs (id),
    progress_id serial references progresses (id)
  );

CREATE TABLE
  completed_iterations (
    id serial primary key not null,
    completed_at bigint,
    iteration_id serial references iterations (id),
    participant_id serial references participants (id)
  );

CREATE TABLE
  completed_backlogs (
    id serial primary key not null,
    completed_at bigint,
    backlog_id serial references backlogs (id)
  );

CREATE TABLE
  completed_tasks (
    id serial primary key not null,
    completed_at bigint,
    task_id serial references tasks (id)
  );


INSERT INTO progresses (name) VALUES ('Undone'), ('Partially Done'), ('Done');
INSERT INTO backlog_types (name) VALUES ('Fixed'), ('Flexible');
INSERT INTO participants (alias, username, password) VALUES ('@admin','admin', '123')
