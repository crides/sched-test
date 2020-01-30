use rlua::prelude::*;
use rlua_serde::*;

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
