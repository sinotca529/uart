use super::keybind::KeySequence;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    keybind: KeyBindings,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let cfg = std::fs::read("config.toml")?;
        let cfg = std::str::from_utf8(&cfg)?;
        toml::from_str(cfg).map_err(Into::into)
    }
}

#[derive(Deserialize, Debug)]
struct KeyBindings {
    make_rect: HashMap<KeySequence, super::mode::make_rect::Op>,
}

