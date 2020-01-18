#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod storage;
mod script;

fn main() {
    let mut script_ctx = script::ScriptContext::new();
    script_ctx.init_lua();
    script_ctx.repl();
}
