//! motors is a super small template library,
//! inspired by [mote](https://github.com/soveran/mote).
//!
//! It's so small right now, all it can do is interpolate string variables.
//!
//! ## Basic usage
//!
//! ```rust
//! use motors;
//! let tmpl = motors::Template::parse("Hello {{user}}!").unwrap();
//! let mut ctx = HashMap::new();
//!
//! ctx.insert("user", "Ada");
//! assert_eq!("Hello Ada!", tmpl.render(&ctx));
//! ```

#![feature(plugin,str_char,collections)]
#![plugin(peg_syntax_ext)]

use std::collections::HashMap;
use Data::*;

#[derive(Debug,Clone)]
pub enum Data {
    Variable(String),
    Text(String),
}

peg_file! motors("motors.rustpeg");

pub struct Template {
    data: Vec<Data>
}

impl Template {
    pub fn parse(input: &str) -> Result<Template,motors::ParseError> {
        match motors::template(input) {
            Ok(t) => Ok(Template { data: t }),
            Err(e) => Err(e)
        }
    }

    pub fn render(&self, context: &HashMap<&str,&str>) -> String {
        let mut out = String::new();

        for elem in &self.data {
            match *elem {
                Text(ref t) => out = out + &t,
                Variable(ref v) => {
                    if let Some(val) = context.get(&v[..]) {
                        out = out + val;
                    }
                }
            };
        }
        out
    }

    pub fn motors(input: &str, context: &HashMap<&str,&str>) -> Result<String,motors::ParseError> {
        Template::parse(input).map(|t| t.render(context))
    }
}

#[test]
fn it_renders_text() {
    let t = Template::parse("Hello World!").unwrap();
    let ctx = HashMap::new();

    assert_eq!("Hello World!", t.render(&ctx));
}

#[test]
fn it_renders_variables() {
    let t = Template::parse("{{user}}").unwrap();
    let mut ctx = HashMap::new();

    assert_eq!("", t.render(&ctx));

    ctx.insert("user", "Ada");
    assert_eq!("Ada", t.render(&ctx));
}

#[test]
fn it_renders_mixed() {
    let t = Template::parse("Hello {{user}}!").unwrap();
    let mut ctx = HashMap::new();

    ctx.insert("user", "Ada");
    assert_eq!("Hello Ada!", t.render(&ctx));
}

#[test]
fn shortcut_works() {
    let mut ctx = HashMap::new();
    ctx.insert("user", "Ada");

    assert_eq!("Hello Ada!",
               Template::motors("Hello {{user}}!", &ctx).unwrap());
}
