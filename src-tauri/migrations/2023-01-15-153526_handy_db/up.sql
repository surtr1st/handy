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
    created_date date not null,
    end_date date not null
  );

CREATE TABLE
  backlog_types (
    id serial primary key not null,
    name text not null,
    backlog_id int null
  );

CREATE TABLE
  backlogs (
    id serial primary key not null,
    title text not null,
    description text,
    goals text not null,
    priority int not null,
    hours int,
    points int,
    created_date date not null,
    iteration_id serial references iterations (id),
    progress_id serial references progresses (id),
    type_id serial references backlog_types (id)
  );

CREATE TABLE
  participants (
    id text primary key not null,
    name text not null,
    backlog_id serial references backlogs (id),
    iteration_id serial references iterations (id)
  );

CREATE TABLE
  tasks (
    id serial primary key not null,
    name text not null,
    created_date bigint,
    hours int,
    actual_hours int,
    progress text not null,
    mode boolean not null,
    pic text references participants (id),
    backlog_id serial references backlogs (id),
    progress_id serial references progresses (id)
  );

CREATE TABLE
  worklogs (
    id serial primary key not null,
    task_id serial references tasks (id),
    participant_id text references participants (id)
  );

CREATE TABLE
  criteria_acceptances (
    id serial primary key not null,
    title text not null,
    status boolean,
    backlog_id serial references backlogs (id)
  );


INSERT INTO progresses (name) VALUES ('Undone'), ('Partially Done'), ('Done');
INSERT INTO backlog_types (name, backlog_id) VALUES ('Fixed', null), ('Flexible', null);
