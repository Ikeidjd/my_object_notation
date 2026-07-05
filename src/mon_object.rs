use std::{collections::HashMap, fmt::Display};

pub enum MonObject {
    Number(String),
    Bool(String),
    String(String),
    Enum(String, Box<MonObject>),
    Array(Vec<MonObject>),
    Dictionary(HashMap<String, MonObject>),
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
            MonObject::Number(s) => write!(f, "{s}"),
            MonObject::Bool(s) => write!(f, "{s}"),
            MonObject::String(s) => write!(f, "{s}"),
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
