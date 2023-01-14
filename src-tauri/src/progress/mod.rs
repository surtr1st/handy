use postgres::{Client, NoTls};
use std::vec::Vec;

pub struct Progress {
    pub id: i32,
    pub name: String,
}

pub fn select_all() -> Vec<Progress> {
    let mut client =
        Client::connect("host=localhost user=postgres password=postgres", NoTls).unwrap();
    let mut list = Vec::new();
    for row in client
        .query("SELECT id, name FROM progresses", &[])
        .unwrap()
    {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        list.push(Progress { id, name });
    }
    list
}
