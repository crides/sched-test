#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod storage;
mod script;

fn main() {
    let ctx = script::ScriptContext::new();
    if let Err(e) = ctx.init() {
        eprintln!("{}", e);
        return;
    };
    ctx.repl();
}
