use std::{collections::HashMap, error::Error, fmt::{Debug, Display}, rc::Rc};

use crate::{lexer, parser};

#[derive(PartialEq, Eq)]
pub enum MonObject {
    PrimitiveType(String),
    String(String),
    Enum(String, Box<MonObject>),
    Array(Vec<MonObject>),
    Dictionary(HashMap<String, MonObject>),
}

impl MonObject {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let source_code = std::fs::read_to_string(path)?;
        let source_code: Rc<[char]> = Rc::from(source_code.chars().collect::<Vec<char>>());

        let tokens = lexer::lex(source_code)?;

        Ok(parser::parse(tokens)?)
    }
}

impl Debug for MonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{self}");
        let s = s.replace(" ", "");
        write!(f, "{}", s.replace("\n", ""))
    }
}

impl Display for MonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = f.width().unwrap_or(0);

        if f.align().is_none() {
            for _ in 0..indent {
                write!(f, " ")?;
            }
        }

        match self {
            MonObject::PrimitiveType(s) => write!(f, "{s}"),
            MonObject::String(s) => write!(f, "\"{s}\""),
            MonObject::Enum(name, mon_object) => write!(f, "{name} {mon_object:<indent$}"),
            MonObject::Array(array) => {
                if array.len() == 0 {
                    return write!(f, "[]");
                }

                write!(f, "[\n")?;

                for object in array {
                    write!(f, "{object:indent$},\n", indent = indent + 4)?;
                }

                for _ in 0..indent {
                    write!(f, " ")?;
                }

                write!(f, "]")
            }
            MonObject::Dictionary(dictionary) => {
                if dictionary.len() == 0 {
                    return write!(f, "{{}}");
                }

                write!(f, "{{\n")?;

                for (key, value) in dictionary {
                    for _ in 0..(indent + 4) {
                        write!(f, " ")?;
                    }

                    write!(f, "{key} = {value:<indent$},\n", indent = indent + 4)?;
                }

                for _ in 0..indent {
                    write!(f, " ")?;
                }

                write!(f, "}}")
            }
        }
    }
}
