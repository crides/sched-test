use sched_test::*;
use dirs::home_dir;

fn main() {
    let ctx = script::ScriptContext::new();
    if let Err(e) = ctx.init() {
        eprintln!("{}", e);
        return;
    };
    let mut init_file = home_dir().unwrap();
    init_file.push(".config/sched/init.lua");
    if let Err(e) = ctx.run_file(init_file) {
        eprintln!("{}", e);
        return;
    };
    // ctx.repl();
}
