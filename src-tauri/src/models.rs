use diesel::prelude::*;

#[derive(Queryable)]
pub struct BacklogType {
    id: i32,
    name: String,
}

#[derive(Queryable)]
pub struct Progress {
    id: i32,
    name: String,
}
