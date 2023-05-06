use super::command::Command;
use crossterm::event::{self, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

enum State {
    Normal,
    Cmd(String),
}

impl State {
    fn trans(&mut self, keycode: &KeyCode) -> Command {
        match &self {
            State::Normal => self.trans_from_normal(keycode),
            State::Cmd(_) => self.trans_from_cmd(keycode),
        }
    }

    fn trans_from_normal(&mut self, keycode: &KeyCode) -> Command {
        match keycode {
            KeyCode::Char(':') => {
                *self = Self::Cmd(String::from(":"));
                Command::Nop
            }
            _ => Command::Nop,
        }
    }

    fn trans_from_cmd(&mut self, keycode: &KeyCode) -> Command {
        let Self::Cmd(cmd) = self else {
            unreachable!();
        };

        match keycode {
            KeyCode::Enter => {
                if cmd == ":q" {
                    Command::Quit
                } else {
                    *self = Self::Normal;
                    Command::Nop
                }
            }
            KeyCode::Char(c) => {
                cmd.push(*c);
                Command::Nop
            }
            KeyCode::Backspace => {
                cmd.pop();
                if cmd.is_empty() {
                    *self = Self::Normal;
                }
                Command::Nop
            }
            _ => Command::Nop,
        }
    }
}

pub struct CommandStream(State);

impl CommandStream {
    pub fn new() -> Self {
        Self(State::Normal)
    }

    pub fn next(&mut self) -> Command {
        event::read()
            .map(|op| {
                if let event::Event::Key(k) = op {
                    self.0.trans(&k.code)
                } else {
                    Command::Nop
                }
            })
            .unwrap()
    }
}

impl Widget for &CommandStream {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let text;
        let fg_color;
        match &self.0 {
            State::Normal => {
                text = "Begin command by ':'";
                fg_color = Color::Rgb(128, 128, 128);
            }
            State::Cmd(m) => {
                text = m;
                fg_color = Color::White;
            }
        }

        let cmd_line = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Rgb(0, 0, 0)));
        let msg = Paragraph::new(Text::raw(text))
            .block(cmd_line)
            .style(Style::default().fg(fg_color).bg(Color::Rgb(0, 0, 0)))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });
        msg.render(area, buf);
    }
}
