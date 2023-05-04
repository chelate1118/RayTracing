use std::env;

use serde::{Serialize, Deserialize};

use crate::loader::FromValue;

pub(crate) struct Config {
    pub(crate) reflect_count: u32,
    pub(crate) render_count: usize,
    pub(crate) export_frame: usize,
    pub(crate) bright: f32,
    pub(crate) file_name: String
}

impl FromValue for Config {
    fn from_value(value: serde_json::Value) -> serde_json::Result<Self> {
        let ci: ConfigInfo = serde_json::from_value(value)?;

        Ok(Config {
            reflect_count: ci.reflect,
            render_count: ci.render,
            export_frame: ci.export,
            bright: ci.bright,
            file_name: env::args().last().expect("File name not provided.z")
        })
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigInfo {
    reflect: u32,
    render: usize,
    export: usize,
    bright: f32,
}