use serde::Deserialize;

use crate::loader::FromValue;

pub(crate) struct Config {
    pub(crate) reflect_count: u32,
    pub(crate) render_count: usize,
    pub(crate) export_frame: usize,
    pub(crate) bright: f32
}

impl FromValue for Config {
    fn from_value(value: serde_json::Value) -> serde_json::Result<Self> {
        let ci: ConfigInfo = serde_json::from_value(value)?;

        Ok(Config {
            reflect_count: ci.reflect,
            render_count: ci.render,
            export_frame: ci.export,
            bright: ci.bright
        })
    }
}

#[derive(Deserialize)]
struct ConfigInfo {
    reflect: u32,
    render: usize,
    export: usize,
    bright: f32,
}