mod api;

use std::{collections::HashMap, sync::Mutex};

use rlua::{prelude::*, Error, MultiValue};
use rustyline::Editor;

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
            globals.set("add_log_type", ctx.create_function(api::add_log_type)?)?;
            globals.set("add_log_types", ctx.create_function(api::add_log_types)?)?;
            globals.set("add_log", ctx.create_function(api::add_log)?)?;
            globals.set("add_log_with_props", ctx.create_function(api::add_log_with_props)?)?;
            globals.set("get_logs", ctx.create_function(api::get_logs)?)?;
            globals.set("set_prop", ctx.create_function(api::set_prop)?)?;
            globals.set("get_props_for", ctx.create_function(api::get_props_for)?)?;
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
                                    .map(|value| format!("{:?}", value))
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
