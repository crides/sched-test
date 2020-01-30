use sched_test::*;

fn main() {
    let ctx = script::ScriptContext::new();
    if let Err(e) = ctx.init() {
        eprintln!("{}", e);
        return;
    };
    ctx.repl();
}
