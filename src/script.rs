use std::sync::{Arc, RwLock};

use fragile::Fragile;
use rlua::{Error, Lua, MultiValue};
use rustyline::Editor;

use crate::storage::LogStorage;

pub struct ScriptContext {
    lua: Lua,
}

impl ScriptContext {
    pub fn new() -> ScriptContext {
        let mut ctx = ScriptContext { lua: Lua::new() };

        ctx.lua.context(|ctx| {
            let globals = ctx.globals();

            let storage = Arc::new(RwLock::new(Fragile::new(LogStorage::new())));
            let lua_storage = Arc::clone(&storage);
            globals
                .set(
                    "add_log",
                    ctx.create_function_mut(move |_, (name, desc): (String, String)| {
                        lua_storage.write().unwrap().get_mut().add_log(&name, &desc);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();

            let lua_storage = Arc::clone(&storage);
            globals
                .set(
                    "add_prop",
                    ctx.create_function_mut(move |_, (id, key, val): (i32, String, String)| {
                        lua_storage
                            .write()
                            .unwrap()
                            .get_mut()
                            .add_prop(id, &key, &val);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();

            let lua_storage = Arc::clone(&storage);
            globals
                .set(
                    "get_logs",
                    ctx.create_function_mut(move |_, ()| {
                        Ok(lua_storage.read().unwrap().get().get_logs())
                    })
                    .unwrap(),
                )
                .unwrap();

            let lua_storage = Arc::clone(&storage);
            globals
                .set(
                    "get_props_for",
                    ctx.create_function_mut(move |_, id: i32| {
                        Ok(lua_storage.read().unwrap().get().get_props_for(id))
                    })
                    .unwrap(),
                )
                .unwrap();
        });
        ctx
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
