use super::keybind::KeySequence;
use crossterm::event::{Event, KeyEvent};
use std::collections::HashMap;

pub struct EventToOp<'a, Op> {
    bindings: &'a HashMap<KeySequence, Op>,
    current_sequence: Vec<KeyEvent>,
}

impl<'a, Op: Clone> EventToOp<'a, Op> {
    pub fn new(bindings: &'a HashMap<KeySequence, Op>) -> Self {
        Self {
            bindings,
            current_sequence: Vec::new(),
        }
    }

    pub fn convert(&mut self, event: Event) -> Option<Op> {
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
