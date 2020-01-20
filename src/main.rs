#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod storage;
mod script;

fn main() {
    script::repl();
}
