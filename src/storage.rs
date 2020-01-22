use std::collections::HashMap;

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
};

pub mod model;
mod schema;

use model::*;
use schema::{logs, attrs};

pub struct LogStorage(SqliteConnection);

impl LogStorage {
    pub fn new() -> LogStorage {
        LogStorage(SqliteConnection::establish("test.db").unwrap())
    }

    pub fn add_log(&mut self, name: &str, desc: &str) -> i32 {
        diesel::insert_into(logs::table)
            .values(&NewLog {
                name,
                desc,
            })
            .execute(&self.0).unwrap();
        logs::table.select(diesel::dsl::max(logs::id)).first::<Option<i32>>(&self.0).unwrap().unwrap()
    }

    pub fn set_prop(&mut self, id: i32, key: &str, val: &str) {
        diesel::insert_into(attrs::table)
            .values(&NewAttr {
                id,
                key,
                val,
            })
            .execute(&self.0).unwrap();
    }

    pub fn add_log_with_props(&mut self, name: &str, desc: &str, props: &HashMap<String, String>) {
        let id = self.add_log(name, desc);
        for (key, val) in props.iter() {
            self.set_prop(id, key, val);
        }
    }

    pub fn get_logs(&self) -> Vec<Log> {
        logs::table.load::<Log>(&self.0).unwrap()
    }

    pub fn get_props_for(&self, id: i32) -> HashMap<String, String> {
        attrs::table.filter(attrs::id.eq(id))
            .load::<Attr>(&self.0).unwrap()
            .into_iter()
            .map(|a| (a.key, a.val))
            .collect()
    }
}
