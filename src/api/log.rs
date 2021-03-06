//! This module contains all the Rust side of the logging API, that is in Rust types and can be
//! easily used by Rust code
use std::{collections::HashMap, sync::Mutex};

use crate::storage::{model::Log, LogStorage};

use super::{error::*, state::API_STATE};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAttr {
    #[serde(default)]
    hidden: bool,
    default: Option<String>,
}

pub type LogAttrs = HashMap<String, LogAttr>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogType {
    name: String,
    attrs: LogAttrs,
}

pub type LogTypes = HashMap<String, HashMap<String, LogAttr>>;

pub fn add_log<S1, S2>(name: S1, desc: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    API_STATE.with(|s| s.lock().unwrap().storage.add_log(name, desc));
}

pub fn add_log_with_props<S1, S2>(name: S1, desc: S2, props: &HashMap<String, String>)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    API_STATE.with(|s| {
        s.lock()
            .unwrap()
            .storage
            .add_log_with_props(name, desc, props)
    });
}

pub fn set_prop<S1, S2>(id: i32, key: S1, val: S2)
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    API_STATE.with(|s| s.lock().unwrap().storage.set_prop(id, key, val));
}

pub fn get_logs() -> Vec<Log> {
    API_STATE.with(|s| s.lock().unwrap().storage.get_logs())
}
pub fn get_props_for(id: i32) -> HashMap<String, String> {
    API_STATE.with(|s| s.lock().unwrap().storage.get_props_for(id))
}

pub fn add_log_type(typ: LogType) {
    API_STATE.with(|s| s.lock().unwrap().log_types.insert(typ.name, typ.attrs));
}

pub fn add_log_types(typ: LogTypes) {
    API_STATE.with(|s| s.lock().unwrap().log_types.extend(typ));
}

pub fn get_log_type<S: AsRef<str>>(key: S) -> Option<LogAttrs> {
    API_STATE.with(|s| s.lock().unwrap().log_types.get(key.as_ref()).cloned())
}

pub fn get_log_types() -> LogTypes {
    API_STATE.with(|s| s.lock().unwrap().log_types.clone())
}

pub fn add_log_with_type<S1, S2>(
    name: S1,
    desc: S2,
    typ: Option<String>,
    props: HashMap<String, String>,
    conform_type: bool,
) -> Result<()>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let mut final_props: HashMap<String, Option<String>> = HashMap::new();
    let conform_type = typ.is_some() && conform_type;
    if let Some(ref typ) = typ {
        let res = API_STATE.with(|s| {
            let api_state = s.lock().unwrap();
            match api_state.log_types.get(typ) {
                Some(type_attrs) => {
                    for (key, attr) in type_attrs {
                        final_props.insert(key.into(), attr.default.clone());
                    }
                    Ok(())
                }
                None => Err(Error {
                    method: "add_log_with_type".into(),
                    kind: ErrorKind::InvalidLogType(typ.into()),
                }),
            }
        });
        if res.is_err() {
            return res;
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

    if let Some(ref typ) = typ {
        for (key, val) in final_props.iter() {
            if val.is_none() {
                return Err(Error {
                    method: "add_log_with_type".into(),
                    kind: ErrorKind::MissingField {
                        typ: typ.into(),
                        field: key.into(),
                    },
                });
            }
        }
    }
    let final_props = final_props
        .into_iter()
        .map(|(k, v)| (k, v.unwrap()))
        .collect();
    add_log_with_props(name, desc, &final_props);
    Ok(())
}
