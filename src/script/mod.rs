mod lua;

use rlua::{prelude::*, Error, MultiValue};
use rustyline::Editor;

use crate::api;
use lua::format_value;

pub struct ScriptContext {
    lua: Lua,
}

impl ScriptContext {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn init(&self) -> LuaResult<()> {
        self.lua.context(|ctx| {
            let globals = ctx.globals();
            globals.set(
                "add_log_type",
                ctx.create_function(|_, lt| Ok(api::add_log_type(lt)))?,
            )?;
            globals.set(
                "add_log_types",
                ctx.create_function(|_, lts| Ok(api::add_log_types(lts)))?,
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
            let mut editor = Editor::<()>::new();
            loop {
                let mut prompt = "> ";
                let mut line = String::new();
                loop {
                    match editor.readline(prompt) {
                        Ok(input) => line.push_str(&input),
                        Err(_) => return,
                    }

                    match ctx.load(&line).eval::<MultiValue>() {
                        Ok(values) => {
                            editor.add_history_entry(line);
                            println!(
                                "{}",
                                values
                                    .iter()
                                    .map(|value| format_value(value))
                                    .collect::<Vec<_>>()
                                    .join("\t")
                            );
                            break;
                        }
                        Err(Error::SyntaxError {
                            incomplete_input: true,
                            ..
                        }) => {
                            // continue reading input and append it to `line`
                            line.push_str("\n"); // separate input lines
                            prompt = ">> ";
                        }
                        Err(e) => {
                            eprintln!("error: {}", e);
                            if let LuaError::CallbackError { cause: c, .. } = e {
                                println!("Caused by: {}", c);
                            }
                            break;
                        }
                    }
                }
            }
        });
    }
}
