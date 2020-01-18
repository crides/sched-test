use chrono::naive::NaiveDateTime;
use rlua::{
    prelude::*,
    Context,
    Value,
};
use rlua_serde::to_value;

use super::schema::{logs, attrs};

#[derive(Queryable, Serialize)]
pub struct Log {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub time: NaiveDateTime,
}

impl<'lua> ToLua<'lua> for Log {
    fn to_lua(self, lua: Context<'lua>) -> LuaResult<Value<'lua>> {
        to_value(lua, self)
    }
}

#[derive(Queryable)]
pub struct Attr {
    pub attr_id: i32,
    pub id: i32,
    pub key: String,
    pub val: String,
}

#[derive(Insertable)]
#[table_name = "logs"]
pub struct NewLog<'a> {
    pub name: &'a str,
    pub desc: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "attrs"]
pub struct NewAttr<'a> {
    pub id: i32,
    pub key: &'a str,
    pub val: &'a str,
}
