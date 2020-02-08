use std::fs::read_to_string;
use std::io::Error as IoError;
use std::path::Path;

use either::*;
use rlua::prelude::*;

use super::lua;
use crate::api;

pub struct ScriptContext {
    lua: Lua,
}

impl ScriptContext {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn init_user<P: AsRef<Path>>(
        &self,
        config_dir: P,
    ) -> Result<(), Either<IoError, LuaError>> {
        let config_dir = config_dir.as_ref();
        let init_file = config_dir.join("init.lua");
        let code = read_to_string(&init_file).map_err(Left)?;
        self.lua
            .context(|ctx| {
                let globals = ctx.globals();
                let package: LuaTable = globals.get("package").unwrap();
                let package_path: String = package.get("path").unwrap();
                let new_package_path = [
                    &package_path,
                    config_dir.join("?.lua").to_str().unwrap(),
                    config_dir.join("?/init.lua").to_str().unwrap(),
                ]
                .join(";");
                package.set("path", new_package_path).unwrap();
                ctx.load(&code)
                    .set_name(init_file.to_str().unwrap())
                    .unwrap()
                    .exec()
            })
            .map_err(Right)
    }

    pub fn init_lib(&self) -> LuaResult<()> {
        self.lua.context(|ctx| {
            let globals = ctx.globals();
            globals.set(
                "pprint",
                ctx.create_function(|ctx, lt| Ok(lua::pprint(&lt, &ctx)))?,
            )?;
            globals.set("repl", ctx.create_function(|ctx, ()| Ok(lua::repl(ctx)))?)?;
            globals.set(
                "readline",
                ctx.create_function(|_, p| Ok(lua::readline(p)))?,
            )?;
            globals.set(
                "add_log_type",
                ctx.create_function(|_, lt| Ok(api::add_log_type(lt)))?,
            )?;
            globals.set(
                "add_log_types",
                ctx.create_function(|_, lts| Ok(api::add_log_types(lts)))?,
            )?;
            globals.set(
                "get_log_type",
                ctx.create_function(|_, key: String| Ok(api::get_log_type(key)))?,
            )?;
            globals.set(
                "get_log_types",
                ctx.create_function(|_, ()| Ok(api::get_log_types()))?,
            )?;
            globals.set(
                "add_log",
                ctx.create_function(|_, (s1, s2): (String, String)| Ok(api::add_log(s1, s2)))?,
            )?;
            globals.set(
                "add_log_with_props",
                ctx.create_function(|_, (s1, s2, p): (String, String, _)| {
                    Ok(api::add_log_with_props(s1, s2, &p))
                })?,
            )?;
            globals.set(
                "get_logs",
                ctx.create_function(|_, ()| Ok(api::get_logs()))?,
            )?;
            globals.set(
                "set_prop",
                ctx.create_function(|_, (id, k, v): (_, String, String)| {
                    Ok(api::set_prop(id, k, v))
                })?,
            )?;
            globals.set(
                "get_props_for",
                ctx.create_function(|_, id| Ok(api::get_props_for(id)))?,
            )?;
            globals.set(
                "add_log_with_type",
                ctx.create_function(|_, (s1, s2, t, p, c): (String, String, _, _, _)| {
                    api::add_log_with_type(s1, s2, t, p, c).map_err(|e| e.into())
                })?,
            )?;
            Ok(())
        })
    }

    pub fn repl(&self) {
        self.lua.context(|ctx| {
            lua::repl(ctx);
        });
    }
}
