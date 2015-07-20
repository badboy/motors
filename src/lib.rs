//! motors is a super small template library,
//! inspired by [mote](https://github.com/soveran/mote).
//!
//! It's so small right now, all it can do is interpolate string variables.
//!
//! ## Basic usage
//!
//! ```rust
//! use motors;
//! use std::collections::HashMap;
//! let tmpl = motors::Template::parse("Hello {{user}}!").unwrap();
//! let mut ctx = HashMap::new();
//!
//! ctx.insert("user", "Ada");
//! assert_eq!("Hello Ada!", tmpl.render(&ctx));
//! ```

#![feature(plugin,vec_push_all)]
#![plugin(peg_syntax_ext)]

use std::collections::HashMap;
use Data::*;

#[derive(Debug,Clone)]
pub enum Data {
    Variable(String),
    Text(String),
    Condition(String, Vec<Data>, Option<Vec<Data>>),
    Comment(String),
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
            self.render_elem(&mut out, elem, context);
        }
        out
    }

    fn render_elem(&self, out: &mut String, elem: &Data, context: &HashMap<&str,&str>) {
        match *elem {
            Text(ref t) => out.push_str(&t),
            Variable(ref v) => {
                if let Some(val) = context.get(&v[..]) {
                    out.push_str(val);
                }
            },
            Condition(ref v, ref left, ref right) => {
                if let Some(_) = context.get(&v[..]) {
                    for el in left {
                        self.render_elem(out, el, context);
                    }
                    out.push_str("\n");
                } else {
                    if let Some(ref right_v) = *right {
                        for el in right_v {
                            self.render_elem(out, el, context);
                        }
                        out.push_str("\n");
                    }
                }
            }
            _ => (),
        };
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

#[test]
fn parses_code_comment_line() {
    let tmpl = Template::parse("% # comment");
    assert!(tmpl.is_ok());
}

#[test]
fn parses_code_line() {
    let s = r#"
% if user
 Hi {{user}}
% else
 this is the else branch
% end

% if userx
if teil
% end
stop

% if userx
if part 2
% else
else part 2
% end
"#;

    let tmpl = Template::parse(s).unwrap();
    let mut ctx = HashMap::new();
    ctx.insert("user", "Ada");
    assert_eq!(" Hi Ada\nstopelse part 2\n", tmpl.render(&ctx));
}
