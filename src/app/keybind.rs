use anyhow::{Error, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use std::convert::TryFrom;

peg::parser! {
    grammar keybind_parser() for str {
        pub rule key_sequence() -> Vec<KeyEvent>
            = key_event()+

        rule key_event() -> KeyEvent
            = keycode:single_key_code() { KeyEvent::new(keycode, KeyModifiers::NONE) }
            / c:$(['A'..='Z']) { KeyEvent::new(KeyCode::Char(c.chars().next().unwrap()), KeyModifiers::SHIFT) }
            / "<" modifiers:modifiers() c:$(['a'..='z' | 'A'..='Z' | '0'..='9']) ">" { KeyEvent::new(KeyCode::Char(c.to_lowercase().chars().next().unwrap()), modifiers) }
            / "<" modifiers:modifiers() k:special_key_code() ">" { KeyEvent::new(k, modifiers) }

        rule single_key_code() -> KeyCode
            = "<" key:special_key_code() ">" { key }
            / c:$(['a'..='z' | '0'..='9']) { KeyCode::Char(c.chars().next().unwrap()) }
            / c:$(['A'..='Z']) { KeyCode::Char(c.chars().next().unwrap()) }

        rule special_key_code() -> KeyCode
            = "Esc" { KeyCode::Esc }
            / "CR" { KeyCode::Enter }
            / "BS" { KeyCode::Backspace }
            / "Space" { KeyCode::Char(' ') }
            / "Up" { KeyCode::Up }
            / "Down" { KeyCode::Down }
            / "Left" { KeyCode::Left }
            / "Right" { KeyCode::Right }
            / "lt" { KeyCode::Char('<') }

        rule modifiers() -> KeyModifiers
            = modifiers:modifier()+ { modifiers.iter().fold(KeyModifiers::empty(), |acc, &m| acc | m) }

        rule modifier() -> KeyModifiers
            = "C-" { KeyModifiers::CONTROL }
            / "S-" { KeyModifiers::SHIFT }
            / "M-" { KeyModifiers::ALT }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct KeySequence(pub Vec<KeyEvent>);

impl KeySequence {
    pub fn new(keys: Vec<KeyEvent>) -> Self {
        Self(keys)
    }
}

impl TryFrom<&str> for KeySequence {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(KeySequence(keybind_parser::key_sequence(s)?))
    }
}

impl<'de> Deserialize<'de> for KeySequence {
    fn deserialize<D>(deserializer: D) -> std::result::Result<KeySequence, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        KeySequence::try_from(&s[..]).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_sequence_from_str() {
        // 単純な小文字
        assert_eq!(
            KeySequence::try_from("h").unwrap().0,
            vec![KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty())]
        );

        // 大文字（Shiftキー）
        assert_eq!(
            KeySequence::try_from("A").unwrap().0,
            vec![KeyEvent::new(KeyCode::Char('a'), KeyModifiers::SHIFT)]
        );

        // Ctrlキー
        assert_eq!(
            KeySequence::try_from("<C-h>").unwrap().0,
            vec![KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL)]
        );

        // 複数キーの組み合わせ
        assert_eq!(
            KeySequence::try_from("gh<C-h>").unwrap().0,
            vec![
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::empty()),
                KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty()),
                KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
            ]
        );

        assert_eq!(
            KeySequence::try_from("<C-h>gh").unwrap().0,
            vec![
                KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::empty()),
                KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty()),
            ]
        );
    }
}

