use std::{collections::HashMap, sync::Mutex, sync::Arc, fmt, error::Error};

use rlua::prelude::*;
use rlua_serde::from_value;

use crate::storage::{
    LogStorage,
    model::*,
};

#[derive(Debug, Deserialize)]
pub struct CustomLogAttr {
    #[serde(default)]
    hidden: bool,
    default: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CustomLogType {
    name: String,
    attrs: HashMap<String, CustomLogAttr>,
}

pub type CustomLogTypes = HashMap<String, HashMap<String, CustomLogAttr>>;

impl<'lua> FromLua<'lua> for CustomLogType {
    fn from_lua(value: LuaValue<'lua>, _lua: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

impl<'lua> FromLua<'lua> for CustomLogAttr {
    fn from_lua(value: LuaValue<'lua>, _lua: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

pub struct LuaAPIState {
    log_types: HashMap<String, HashMap<String, CustomLogAttr>>,
    storage: LogStorage,
}

#[derive(Debug)]
pub struct APIError {
    function: String,
    message: String,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in API: {}: {}", self.function, self.message)
    }
}

impl Error for APIError {}

impl APIError {
    pub fn new<S1: Into<String>, S2: Into<String>>(function: S1, message: S2) -> LuaError {
        LuaError::ExternalError(Arc::new(APIError {
            function: function.into(),
            message: message.into(),
        }))
    }
}

lazy_static! {
    pub static ref API_STATE: Mutex<LuaAPIState> = Mutex::new(LuaAPIState {
        log_types: HashMap::new(),
        storage: LogStorage::new(),
    });
}

pub fn add_log_type(_: LuaContext, typ: CustomLogType) -> LuaResult<()> {
    API_STATE
        .lock()
        .unwrap()
        .log_types
        .insert(typ.name, typ.attrs);
    println!("{:#?}", API_STATE.lock().unwrap().log_types);
    Ok(())
}

pub fn add_log_types(_: LuaContext, typ: CustomLogTypes) -> LuaResult<()> {
    API_STATE
        .lock()
        .unwrap()
        .log_types
        .extend(typ);
    println!("{:#?}", API_STATE.lock().unwrap().log_types);
    Ok(())
}

pub fn add_log(_: LuaContext, (name, desc): (String, String)) -> LuaResult<()> {
    API_STATE.lock().unwrap().storage.add_log(&name, &desc);
    Ok(())
}

pub fn add_log_raw<S1, S2>(name: S1, desc: S2, typ: Option<String>, props: HashMap<String, String>, conform_type: bool) -> LuaResult<()> 
where S1: AsRef<str>, S2: AsRef<str> {
    let mut final_props: HashMap<String, Option<String>> = HashMap::new();
    let conform_type = typ.is_some() && conform_type;
    if let Some(ref typ) = typ {
        let api_state = API_STATE.lock().unwrap();
        match api_state.log_types.get(typ) {
            Some(type_attrs) => {
                for (key, attr) in type_attrs {
                    final_props.insert(key.into(), attr.default.clone());
                }
            },
            None => return Err(APIError::new("add_log_raw", format!("no such log type '{}'", typ))),
        }
    }

    if conform_type {
        for (key, val) in props {
            final_props.entry(key).and_modify(|v| *v = Some(val));
        }
    } else {
        for (key, val) in props {
            final_props.insert(key, Some(val));
        }
    }
    for (key, val) in final_props.iter() {
        if val.is_none() {
            return Err(APIError::new("add_log_raw", format!("field '{}' is missing", key)));
        }
    }
    Ok(())
}

pub fn lua_add_log_raw(_: LuaContext, table: LuaTable) -> LuaResult<()> {
    if !table.contains_key("name").unwrap() {
        return Err(APIError::new("lua_add_log_raw", "field 'name' is required but missing"));
    }
    Ok(())
}

pub fn add_log_with_props(_: LuaContext, (name, desc, props): (String, String, HashMap<String, String>)) -> LuaResult<()> {
    API_STATE.lock().unwrap().storage.add_log_with_props(&name, &desc, &props);
    Ok(())
}

pub fn set_prop(_: LuaContext, (id, key, val): (i32, String, String)) -> LuaResult<()> {
    API_STATE.lock().unwrap().storage.set_prop(id, &key, &val);
    Ok(())
}

pub fn get_logs(_: LuaContext, (): ()) -> LuaResult<Vec<Log>> {
    Ok(API_STATE.lock().unwrap().storage.get_logs())
}
pub fn get_props_for(_: LuaContext, id: i32) -> LuaResult<HashMap<String, String>> {
    Ok(API_STATE.lock().unwrap().storage.get_props_for(id))
}
