use std::collections::HashMap;
use std::sync::Mutex;

use super::{event::*, log::*};
use crate::storage::Storage;

pub struct APIState<'lua> {
    pub(crate) storage: LogStorage,
    pub(crate) log_types: LogTypes,
}

lazy_static! {
    pub static API_STATE: Mutex<APIState<'static>> = Mutex::new(APIState {
        storage: Storage::new(),
        log_types: HashMap::new(),
    });
}
