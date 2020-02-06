use std::fs;

use dirs::config_dir;

use sched_test::*;

fn main() {
    let ctx = script::ScriptContext::new();
    if let Err(e) = ctx.init() {
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
    let init_file = config_dir.join("init.lua");
    if let Err(e) = ctx.run_init_file(init_file) {
        eprintln!("{}", e);
        return;
    };
    // ctx.repl();
}
