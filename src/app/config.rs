use file_config::FileConfig;

use super::{
    keybind::KeySequence,
    mode::{make_path, make_rect, make_text, normal, select},
};
use anyhow::Result;
use std::collections::HashMap;

mod file_config;

#[derive(Debug)]
pub struct Config {
    keybind: KeyBindings,
}

impl Config {
    pub fn keybind(&self) -> &KeyBindings {
        &self.keybind
    }

    pub fn load() -> Result<Self> {
        let file_config = FileConfig::load()?;
        Ok(file_config.into())
    }
}

#[derive(Debug)]
pub struct KeyBindings {
    pub normal: HashMap<KeySequence, normal::Op>,
    pub select: HashMap<KeySequence, select::Op>,
    pub make_rect: HashMap<KeySequence, make_rect::Op>,
    pub make_text: HashMap<KeySequence, make_text::Op>,
    pub make_path: HashMap<KeySequence, make_path::Op>,
}
