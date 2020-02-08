use std::fs;

use dirs::config_dir;

use sched_test::*;

fn main() {
    let ctx = script::ScriptContext::new();
    if let Err(e) = ctx.init_lib() {
        eprintln!("{}", e);
        return;
    };
    let config_dir = config_dir().unwrap().join("sched");
    if !config_dir.is_dir() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("{}", e);
            return;
        }
    }
    if let Err(e) = ctx.init_user(config_dir) {
        eprintln!("{}", e);
        return;
    };
    // ctx.repl();
}
