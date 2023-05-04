use serde_json::*;
use std::fs;

pub(super) fn load_map(file_name: &str) -> crate::map::Map {
    crate::map::Map::from_value(
        file_to_value(&format!("maps/{file_name}.json"))
            .unwrap_or_else(|_| panic!("File {file_name} doesn't exist."))
    ).expect("Fail to deserialize map informations.")
}

pub(crate) fn file_to_value(file_path: &str) -> Result<Value> {
    str_to_value(&fs::read_to_string(file_path).unwrap())
}

pub(crate) fn str_to_value(str: &str) -> Result<Value> {
    from_str(str)
}

pub(crate) trait FromValue {
    fn from_value(value: Value) -> Result<Self>
        where Self: Sized;
}