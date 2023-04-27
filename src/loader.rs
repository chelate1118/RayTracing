use serde_json::*;
use std::fs;

pub(crate) fn str_to_value(str: &str) -> Result<Value> {
    from_str(str)
}

pub(crate) fn file_to_value(file_path: &str) -> Result<Value> {
    str_to_value(&fs::read_to_string(file_path).unwrap())
}

pub(crate) trait FromValue {
    fn from_value(value: Value) -> Result<Self>
        where Self: Sized;
}