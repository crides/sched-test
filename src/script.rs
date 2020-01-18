use std::sync::{Arc, RwLock};

use rlua::{Lua, Error, MultiValue};
use rustyline::Editor;

use crate::storage::LogStorage;

pub struct ScriptContext {
    lua: Lua,
    storage: Arc<RwLock<LogStorage>>,
}

impl ScriptContext {
    pub fn new() -> ScriptContext {
        ScriptContext {
            lua: Lua::new(),
            storage: Arc::new(RwLock::new(LogStorage::new())),
        }
    }

    pub fn init_lua(&mut self) {
        self.lua.context(|ctx| {
            let globals = ctx.globals();

            let lua_storage = Arc::clone(&self.storage);
            let add_log = ctx.create_function_mut(|_, (name, desc): (String, String)| {
                dbg!(name, desc);
                // lua_storage.write().unwrap().add_log(&name, &desc);
                Ok(())
            }).unwrap();
            globals.set("add_log", add_log).unwrap();

            let lua_storage = Arc::clone(&self.storage);
            let add_prop = ctx.create_function_mut(|_, (id, key, val): (i32, String, String)| {
                dbg!(id, key, val);
                // lua_storage.write().unwrap().add_prop(id, &key, &val);
                Ok(())
            }).unwrap();
            globals.set("add_prop", add_prop).unwrap();

            let lua_storage = Arc::clone(&self.storage);
            let get_logs = ctx.create_function_mut(|_, ()| {
                println!("get_logs");
                // Ok(lua_storage.read().unwrap().get_logs())
                Ok(())
            }).unwrap();
            globals.set("get_logs", get_logs).unwrap();

            let lua_storage = Arc::clone(&self.storage);
            let get_props_for = ctx.create_function_mut(|_, id: i32| {
                println!("get_props_for");
                dbg!(id);
                // Ok(lua_storage.read().unwrap().get_props_for(id))
                Ok(())
            }).unwrap();
            globals.set("get_props_for", get_props_for).unwrap();

        });
    }
    
    pub fn repl(&mut self) {
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
                            break;
                        }
                    }
                }
            }
        });
    }
}
