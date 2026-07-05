use std::collections::HashMap;

use crate::mon_object::MonObject;

pub trait Serializable {
    fn serialize(&self) -> MonObject;
}

macro_rules! serialize_primitive_type {
    ($type_name:ty) => {
        impl Serializable for $type_name {
            fn serialize(&self) -> MonObject {
                MonObject::PrimitiveType(self.to_string())
            }
        }
    };
    ($type_name:ty, $($type_names:ty), +) => {
        serialize_primitive_type!($type_name);
        serialize_primitive_type!($($type_names), +);
    }
}

serialize_primitive_type!(u8, u16, u32, u64, u128, usize);
serialize_primitive_type!(i8, i16, i32, i64, i128, isize);
serialize_primitive_type!(bool, char);

impl Serializable for String {
    fn serialize(&self) -> MonObject {
        MonObject::String(self.clone())
    }
}

impl Serializable for str {
    fn serialize(&self) -> MonObject {
        self.to_owned().serialize()
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> MonObject {
        MonObject::Array(self.iter().map(|t| t.serialize()).collect())
    }
}

impl<V: Serializable> Serializable for HashMap<String, V> {
    fn serialize(&self) -> MonObject {
        MonObject::Dictionary(self.iter().map(|(k, v)| (k.clone(), v.serialize())).collect())
    }
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self) -> MonObject {
        match self {
            Self::Some(t) => MonObject::Enum("Some".to_owned(), Box::new(MonObject::Array(vec![t.serialize()]))),
            Self::None => MonObject::Enum("None".to_owned(), Box::new(MonObject::Array(vec![]))),
        }
    }
}

impl<T: Serializable, E: Serializable> Serializable for Result<T, E> {
    fn serialize(&self) -> MonObject {
        match self {
            Self::Ok(t) => MonObject::Enum("Ok".to_owned(), Box::new(MonObject::Array(vec![t.serialize()]))),
            Self::Err(e) => MonObject::Enum("Err".to_owned(), Box::new(MonObject::Array(vec![e.serialize()]))),
        }
    }
}
