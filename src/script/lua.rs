use rlua::prelude::*;
use rlua_serde::*;
use serde_json::{
    Value as JsonValue,
    Number as JsonNumber,
};

use std::collections::HashMap;
use crate::api::{LogAttr, LogType};

impl<'lua> FromLua<'lua> for LogType {
    fn from_lua(value: LuaValue<'lua>, _lua: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

impl<'lua> FromLua<'lua> for LogAttr {
    fn from_lua(value: LuaValue<'lua>, _lua: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

fn lua_to_json<'lua>(v: &LuaValue<'lua>) -> JsonValue {
    match v {
        LuaValue::Nil => JsonValue::Null,
        LuaValue::Boolean(b) => JsonValue::Bool(*b),
        LuaValue::Integer(i) => JsonValue::Number(JsonNumber::from_f64(*i as f64).unwrap()),
        LuaValue::Number(f) => JsonValue::Number(JsonNumber::from_f64(*f).unwrap()),
        LuaValue::String(s) => JsonValue::String(s.to_str().unwrap().into()),
        LuaValue::LightUserData(_) => JsonValue::String("<light-user-data>".into()),
        LuaValue::UserData(_) => JsonValue::String("<user-data>".into()),
        LuaValue::Function(_) => JsonValue::String("<function>".into()),
        LuaValue::Thread(_) => JsonValue::String("<thread>".into()),
        LuaValue::Table(t) => {
            let map = t.clone().pairs::<String, LuaValue>().map(|pair| {
                let (key, val) = pair.unwrap();
                (key, lua_to_json(&val))
            }).collect();
            JsonValue::Object(map)
        },
        LuaValue::Error(e) => JsonValue::Null,
    }
}

pub fn format_value<'lua>(v: &LuaValue<'lua>) -> String {
    let j = lua_to_json(v);
    serde_json::to_string_pretty(&j).unwrap()
}
