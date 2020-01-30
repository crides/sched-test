use std::collections::HashMap;

use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod model;
mod schema;

use model::*;
use schema::{attrs, logs};

pub struct LogStorage(SqliteConnection);

impl LogStorage {
    pub fn new() -> LogStorage {
        LogStorage(SqliteConnection::establish("test.db").unwrap())
    }

    pub fn add_log<S1, S2>(&mut self, name: S1, desc: S2) -> i32
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        diesel::insert_into(logs::table)
            .values(&NewLog {
                name: name.as_ref(),
                desc: desc.as_ref(),
            })
            .execute(&self.0)
            .unwrap();
        logs::table
            .select(diesel::dsl::max(logs::id))
            .first::<Option<i32>>(&self.0)
            .unwrap()
            .unwrap()
    }

    pub fn set_prop<S1, S2>(&mut self, id: i32, key: S1, val: S2)
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        diesel::insert_into(attrs::table)
            .values(&NewAttr {
                id,
                key: key.as_ref(),
                val: val.as_ref(),
            })
            .execute(&self.0)
            .unwrap();
    }

    pub fn add_log_with_props<S1, S2>(
        &mut self,
        name: S1,
        desc: S2,
        props: &HashMap<String, String>,
    ) where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let id = self.add_log(name.as_ref(), desc.as_ref());
        for (key, val) in props.iter() {
            self.set_prop(id, key, val);
        }
    }

    pub fn get_logs(&self) -> Vec<Log> {
        logs::table.load::<Log>(&self.0).unwrap()
    }

    pub fn get_props_for(&self, id: i32) -> HashMap<String, String> {
        attrs::table
            .filter(attrs::id.eq(id))
            .load::<Attr>(&self.0)
            .unwrap()
            .into_iter()
            .map(|a| (a.key, a.val))
            .collect()
    }
}
