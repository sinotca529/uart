use crate::controller::{command_stream::CommandStream, Controller};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

pub struct View {
    ctrl: Controller,
    cmd_stream: CommandStream,
}

impl View {
    pub fn new(ctrl: Controller, cmd_stream: CommandStream) -> Self {
        Self { ctrl, cmd_stream }
    }

    pub fn render(&self, f: &mut Frame<impl Backend>) {
        let model = self.ctrl.model();

        let chunks1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(f.size().height - 3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.size());

        let chunks2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks1[0]);

        f.render_widget(model, chunks2[1]);
        f.render_widget(&self.cmd_stream, chunks1[1]);
    }

    fn main_loop(&mut self, terminal: &mut Terminal<impl Backend>) {
        use crate::controller::command::Command::*;
        loop {
            terminal.draw(|f| self.render(f)).unwrap();

            match self.cmd_stream.next() {
                Quit => break,
                _ => {}
            }
        }
    }

    pub fn run(&mut self) {
        // Setup terminal
        enable_raw_mode().unwrap();
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        // create app and run it
        self.main_loop(&mut terminal);

        // restore terminal
        disable_raw_mode().unwrap();
        execute!(terminal.backend_mut(), LeaveAlternateScreen,).unwrap();
        terminal.show_cursor().unwrap();
    }
}
