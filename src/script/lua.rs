use rlua::prelude::*;
use rlua_serde::*;
use rustyline::Editor;

use crate::api::{LogAttr, LogType};

impl<'lua> FromLua<'lua> for LogType {
    fn from_lua(value: LuaValue<'lua>, _ctx: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

impl<'lua> ToLua<'lua> for LogType {
    fn to_lua(self, ctx: LuaContext<'lua>) -> LuaResult<LuaValue<'lua>> {
        to_value(ctx, self)
    }
}

impl<'lua> FromLua<'lua> for LogAttr {
    fn from_lua(value: LuaValue<'lua>, _ctx: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

impl<'lua> ToLua<'lua> for LogAttr {
    fn to_lua(self, ctx: LuaContext<'lua>) -> LuaResult<LuaValue<'lua>> {
        to_value(ctx, self)
    }
}

pub fn format_value<'lua>(v: &LuaValue<'lua>, ctx: &LuaContext<'lua>) -> String {
    let mut s = String::new();
    let mut formatter = LuaFormatter::new(2);
    formatter.format_value(ctx, v, &mut s);
    s
}

pub fn pprint<'lua>(v: &LuaValue<'lua>, ctx: &LuaContext<'lua>) {
    println!("{}", format_value(v, ctx));
}

pub fn readline(prompt: String) -> LuaResult<String> {
    let mut editor = Editor::<()>::new();
    editor.readline(&prompt).map_err(LuaError::external)
}

struct LuaFormatter {
    indent: usize,
    has_value: bool,
    indent_size: usize,
}

impl LuaFormatter {
    fn new(i: usize) -> Self {
        Self {
            indent: 0,
            has_value: false,
            indent_size: i,
        }
    }

    fn format_value(&mut self, ctx: &LuaContext<'_>, v: &LuaValue<'_>, s: &mut String) {
        match v {
            LuaValue::Nil => s.push_str("nil"),
            LuaValue::Boolean(b) => s.push_str(&b.to_string()),
            LuaValue::Integer(i) => s.push_str(&i.to_string()),
            LuaValue::Number(f) => s.push_str(&f.to_string()),
            LuaValue::String(_s) => {
                s.push_str(&format!("{:?}", _s.to_str().unwrap()));
            }
            LuaValue::LightUserData(d) => s.push_str(&format!("{:?}", d)),
            LuaValue::UserData(d) => s.push_str(&format!("{:?}", d)),
            LuaValue::Function(f) => s.push_str(&format!("{:?}", f)),
            LuaValue::Thread(t) => s.push_str(&format!("{:?}", t)),
            LuaValue::Table(t) => {
                s.push_str("{");
                self.has_value = false;
                self.indent += 1;
                for pair in t.clone().pairs::<LuaValue, LuaValue>() {
                    let (key, val) = pair.unwrap();
                    s.push('\n');
                    s.push_str(&" ".repeat(self.indent * self.indent_size));
                    self.format_value(ctx, &key, s);
                    s.push_str(" = ");
                    self.format_value(ctx, &val, s);
                    self.has_value = true;
                    s.push(',');
                }
                self.indent -= 1;
                if self.has_value {
                    s.push('\n');
                    s.push_str(&" ".repeat(self.indent * self.indent_size));
                }
                s.push_str("}");
            }
            LuaValue::Error(e) => s.push_str(&format!("{:?}", e)),
        }
    }
}
