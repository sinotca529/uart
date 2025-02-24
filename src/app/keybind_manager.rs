use super::keybind::KeySequence;
use anyhow::Result;
use crossterm::event::{Event, KeyEvent};
use std::collections::HashMap;

pub struct KeyBindManager<Op> {
    bindings: HashMap<KeySequence, Op>,
    current_sequence: Vec<KeyEvent>,
}

impl<Op: Clone> KeyBindManager<Op> {
    pub fn new(bindings: HashMap<&str, Op>) -> Result<Self> {
        let mut new_bindings = HashMap::new();
        for (key, op) in bindings {
            new_bindings.insert(KeySequence::try_from(key)?, op);
        }

        Ok(Self {
            bindings: new_bindings,
            current_sequence: Vec::new(),
        })
    }

    pub fn process_event(&mut self, event: Event) -> Option<Op> {
        match event {
            Event::Key(key) => {
                self.current_sequence.push(key);
                let sequence = KeySequence::new(self.current_sequence.clone());

                if let Some(op) = self.bindings.get(&sequence) {
                    self.current_sequence.clear();
                    Some(op.clone())
                } else if self.is_possible_prefix(&sequence) {
                    None
                } else {
                    self.current_sequence.clear();
                    None
                }
            }
            _ => None,
        }
    }

    fn is_possible_prefix(&self, sequence: &KeySequence) -> bool {
        self.bindings.keys().any(|k| k.0.starts_with(&sequence.0))
    }   
}