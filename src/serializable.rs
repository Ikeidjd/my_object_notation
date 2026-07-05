use std::collections::HashMap;

use crate::mon_object::MonObject;

pub trait Serializable {
    fn serialize(&self) -> MonObject;
}

macro_rules! serialize_int {
    ($type_name:ty) => {
        impl Serializable for $type_name {
            fn serialize(&self) -> MonObject {
                MonObject::Number(self.to_string())
            }
        }
    };
    ($type_name:ty, $($type_names:ty), +) => {
        serialize_int!($type_name);
        serialize_int!($($type_names), +);
    }
}

serialize_int!(u8, u16, u32, u64, u128);
serialize_int!(i8, i16, i32, i64, i128);

impl Serializable for String {
    fn serialize(&self) -> MonObject {
        MonObject::String(self.clone())
    }
}

impl Serializable for str {
    fn serialize(&self) -> MonObject {
        MonObject::String(self.to_owned())
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
