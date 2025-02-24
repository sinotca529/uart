use super::{
    keybind::KeySequence,
    mode::{make_path, make_rect, make_text, normal, select},
};
use anyhow::{anyhow, Context as _};
use dirs::config_dir;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    keybind: KeyBindings,
}

impl Config {
    pub fn keybind(&self) -> &KeyBindings {
        &self.keybind
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = config_dir()
            .ok_or(anyhow!("config directory does not found"))?
            .join("uart")
            .join("config.toml");

        // 設定ファイルが存在しない場合は作成する
        if !config_path.exists() {
            std::fs::create_dir_all(config_path.parent().unwrap())?;
            std::fs::write(
                &config_path,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/asset/config.toml")),
            )
            .with_context(|| anyhow!("Failed to copy config.toml"))?;
        }

        let cfg = std::fs::read(&config_path)?;
        let cfg = std::str::from_utf8(&cfg)?;
        toml::from_str(cfg).map_err(Into::into)
    }
}

#[derive(Deserialize, Debug)]
pub struct KeyBindings {
    pub normal: HashMap<KeySequence, normal::Op>,
    pub select: HashMap<KeySequence, select::Op>,
    pub make_rect: HashMap<KeySequence, make_rect::Op>,
    pub make_text: HashMap<KeySequence, make_text::Op>,
    pub make_path: HashMap<KeySequence, make_path::Op>,
}
