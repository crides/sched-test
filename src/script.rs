use std::sync::{Arc, RwLock};

use rlua::{
    Error, MultiValue,
    prelude::*,
};
use rustyline::Editor;

use crate::storage::LogStorage;

struct Scope<'a, 'b, 'c> {
    ctx: LuaContext<'a>,
    globals: LuaTable<'a>,
    scope: &'c LuaScope<'a, 'b>,
}

impl<'a, 'b, 'c> Scope<'a, 'b, 'c> {
    fn add_apis(&self) {
        let storage = Arc::new(RwLock::new(LogStorage::new()));
        let lua_storage = Arc::clone(&storage);
        self.globals
            .set(
                "add_log",
                self.scope.create_function_mut(move |_, (name, desc): (String, String)| {
                    lua_storage.write().unwrap().add_log(&name, &desc);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        let lua_storage = Arc::clone(&storage);
        self.globals
            .set(
                "add_prop",
                self.scope.create_function_mut(move |_, (id, key, val): (i32, String, String)| {
                    lua_storage
                        .write()
                        .unwrap()
                        .add_prop(id, &key, &val);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        let lua_storage = Arc::clone(&storage);
        self.globals
            .set(
                "get_logs",
                self.scope.create_function_mut(move |_, ()| {
                    Ok(lua_storage.read().unwrap().get_logs())
                })
                .unwrap(),
            )
            .unwrap();

        let lua_storage = Arc::clone(&storage);
        self.globals
            .set(
                "get_props_for",
                self.scope.create_function_mut(move |_, id: i32| {
                    Ok(lua_storage.read().unwrap().get_props_for(id))
                })
                .unwrap(),
            )
            .unwrap();
    }

    fn repl(&self) {
        let mut editor = Editor::<()>::new();
        loop {
            let mut prompt = "> ";
            let mut line = String::new();
            loop {
                match editor.readline(prompt) {
                    Ok(input) => line.push_str(&input),
                    Err(_) => return,
                }

                match self.ctx.load(&line).eval::<MultiValue>() {
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
    }
}

pub fn repl() {
    let lua = Lua::new();

    lua.context(|ctx| {
        let globals = ctx.globals();

        ctx.scope(|scope| {
            let scope = Scope {
                ctx,
                globals,
                scope,
            };
            scope.add_apis();
            scope.repl();
        });
    });
}
