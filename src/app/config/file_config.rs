use anyhow::{anyhow, Context as _};
use dirs::config_dir;
use serde::Deserialize;
use std::collections::HashMap;

use super::Config;
use crate::app::{
    keybind::KeySequence,
    mode::{make_path, make_rect, make_text, normal, select},
};

#[derive(Deserialize, Debug)]
pub struct FileConfig {
    keybind: KeyBindings,
}

#[derive(Deserialize, Debug)]
struct KeyBindings {
    common: Common,
    normal: Normal,
    select: Select,
    make_rect: Rect,
    make_path: Path,
    make_text: Text,
}

impl FileConfig {
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

impl From<FileConfig> for Config {
    fn from(val: FileConfig) -> Self {
        use crate::util::Direction::*;

        let kb = val.keybind;
        let Common {
            move_cursor_left,
            move_cursor_up,
            move_cursor_down,
            move_cursor_right,
        } = kb.common;

        let normal = {
            use normal::Op::*;
            let mut m = HashMap::new();
            m.insert(move_cursor_left.clone(), MoveCursor(Left));
            m.insert(move_cursor_up.clone(), MoveCursor(Up));
            m.insert(move_cursor_down.clone(), MoveCursor(Down));
            m.insert(move_cursor_right.clone(), MoveCursor(Right));
            m.insert(kb.normal.make_rect, EnterMakeRect);
            m.insert(kb.normal.make_path, EnterMakePath);
            m.insert(kb.normal.make_text, EnterMakeText);
            m.insert(kb.normal.select_shape, EnterSelectShape);
            m
        };

        let select = {
            use select::Op::*;
            let mut m = HashMap::new();
            m.insert(move_cursor_left.clone(), MoveCursor(Left));
            m.insert(move_cursor_up.clone(), MoveCursor(Up));
            m.insert(move_cursor_down.clone(), MoveCursor(Down));
            m.insert(move_cursor_right.clone(), MoveCursor(Right));
            m.insert(kb.select.move_shape_left.clone(), MoveShapes(Left));
            m.insert(kb.select.move_shape_up.clone(), MoveShapes(Up));
            m.insert(kb.select.move_shape_down.clone(), MoveShapes(Down));
            m.insert(kb.select.move_shape_right.clone(), MoveShapes(Right));
            m.insert(kb.select.delete_shape, DeleteShapes);
            m.insert(kb.select.to_normal_mode, EnterNormalMode);
            m.insert(kb.select.toggle_shape_select, ToggleSelect);
            m
        };

        let make_rect = {
            use make_rect::Op::*;
            let mut m = HashMap::new();
            m.insert(move_cursor_left.clone(), MoveCursor(Left));
            m.insert(move_cursor_up.clone(), MoveCursor(Up));
            m.insert(move_cursor_down.clone(), MoveCursor(Down));
            m.insert(move_cursor_right.clone(), MoveCursor(Right));
            m.insert(kb.make_rect.next_line_style, NextStyle);
            m.insert(kb.make_rect.confirm, MakeShape);
            m
        };

        let make_path = {
            use make_path::Op::*;
            let mut m = HashMap::new();
            m.insert(move_cursor_left.clone(), MoveCursor(Left));
            m.insert(move_cursor_up.clone(), MoveCursor(Up));
            m.insert(move_cursor_down.clone(), MoveCursor(Down));
            m.insert(move_cursor_right.clone(), MoveCursor(Right));
            m.insert(kb.make_path.next_line_style, SelectNextStyle);
            m.insert(kb.make_path.next_arrow_state, SelectNextArrowState);
            m.insert(kb.make_path.confirm, MakeShape);
            m.insert(kb.make_path.back, Back);
            m
        };

        let make_text = {
            use make_text::Op::*;
            let mut m = HashMap::new();
            m.insert(kb.make_text.confirm, MakeText);
            m
        };

        let keybind = super::KeyBindings {
            normal,
            select,
            make_rect,
            make_text,
            make_path,
        };
        Config { keybind }
    }
}

#[derive(Deserialize, Debug)]
struct Common {
    move_cursor_left: KeySequence,
    move_cursor_down: KeySequence,
    move_cursor_up: KeySequence,
    move_cursor_right: KeySequence,
}

#[derive(Deserialize, Debug)]
struct Normal {
    make_rect: KeySequence,
    make_path: KeySequence,
    make_text: KeySequence,
    select_shape: KeySequence,
}

#[derive(Deserialize, Debug)]
struct Select {
    move_shape_left: KeySequence,
    move_shape_down: KeySequence,
    move_shape_up: KeySequence,
    move_shape_right: KeySequence,
    delete_shape: KeySequence,
    to_normal_mode: KeySequence,
    toggle_shape_select: KeySequence,
}

#[derive(Deserialize, Debug)]
struct Rect {
    next_line_style: KeySequence,
    confirm: KeySequence,
}

#[derive(Deserialize, Debug)]
struct Path {
    next_line_style: KeySequence,
    next_arrow_state: KeySequence,
    confirm: KeySequence,
    back: KeySequence,
}

#[derive(Deserialize, Debug)]
struct Text {
    confirm: KeySequence,
}
