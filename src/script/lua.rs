use rlua::prelude::*;
use rlua_serde::*;

use crate::api::{LogAttr, LogType};

impl<'lua> FromLua<'lua> for LogType {
    fn from_lua(value: LuaValue<'lua>, _lua: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

impl<'lua> FromLua<'lua> for LogAttr {
    fn from_lua(value: LuaValue<'lua>, _lua: LuaContext<'lua>) -> LuaResult<Self> {
        from_value(value)
    }
}

pub fn format_value<'lua>(v: &LuaValue<'lua>, ctx: &LuaContext<'lua>) -> String {
    let mut s = String::new();
    let mut formatter = LuaFormatter::new(2);
    formatter.format_value(ctx, v, &mut s);
    s
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
            },
            LuaValue::LightUserData(d) => s.push_str(&format!("{:?}", d)),
            LuaValue::UserData(d) => s.push_str(&format!("{:?}", d)),
            LuaValue::Function(f) => s.push_str(&format!("{:?}", f)),
            LuaValue::Thread(t) => s.push_str(&format!("{:?}", t)),
            LuaValue::Table(t) => {
                s.push_str("{");
                self.has_value = false;
                self.indent += 1;
                for pair in t.clone().pairs::<String, LuaValue>() {
                    let (key, val) = pair.unwrap();
                    if !self.has_value {
                        s.push('\n');
                    } else {
                        s.push_str(",\n");
                    }
                    s.push_str(&" ".repeat(self.indent * self.indent_size));
                    s.push_str(&key);
                    s.push_str(" = ");
                    self.format_value(ctx, &val, s);
                    self.has_value = true;
                }
                self.indent -= 1;
                if self.has_value {
                    s.push('\n');
                    s.push_str(&" ".repeat(self.indent * self.indent_size));
                }
                s.push_str("}");
            },
            LuaValue::Error(e) => s.push_str(&format!("{:?}", e)),
        }
    }
}
