use std::collections::HashMap;
use std::sync::Mutex;

use super::{event::*, log::*};
use crate::storage::LogStorage;

pub struct APIState<'lua> {
    pub(crate) storage: LogStorage,
    pub(crate) log_types: LogTypes,
    pub(crate) events: Vec<EventType<'lua>>,
}

thread_local! {
    pub static API_STATE: Mutex<APIState<'static>> = Mutex::new(APIState {
        storage: LogStorage::new(),
        log_types: HashMap::new(),
        events: Vec::new(),
    });
}
