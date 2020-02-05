use sched_test::*;

fn main() {
    let ctx = script::ScriptContext::new();
    if let Err(e) = ctx.init() {
        eprintln!("{}", e);
        return;
    };
    if let Err(e) = ctx.run_file("/home/steven/.config/sched/init.lua") {
        eprintln!("{}", e);
        return;
    };
    // ctx.repl();
}
