mod canvas;
mod cmd_line;
mod mode;
mod shape;

use self::{canvas::Canvas, cmd_line::CmdLine, mode::ModeHandler, shape::Shape};
use crate::util::{Size, UCoord};
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

pub enum AppOp {
    MakeShape(UCoord, Box<dyn Shape>),
    MoveCanvasCursor(crate::util::Direction),
    SetCanvasCursor(UCoord),
    QuitApp,
    Nop,
}

/// The application
#[derive(Default)]
pub struct App {
    canvas: Canvas,
    mode: ModeHandler,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    fn render(&mut self, f: &mut Frame<impl Backend>) {
        let chunks1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(f.size().height - 1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let canvas_size = Size::new(chunks1[0].width, chunks1[0].height);
        self.canvas.update_rendering_offset(canvas_size);
        f.render_widget(self.canvas.shape_renderer(), chunks1[0]);
        f.render_widget(
            self.mode
                .canvas_modifier(self.canvas.rendering_offset(), self.canvas.cursor().coord()),
            chunks1[0],
        );
        f.render_widget(self.canvas.cursor_renderer(), chunks1[0]);

        let cmd_line = self.mode.get().cmd_line();
        f.render_widget(cmd_line, chunks1[1]);
    }

    /// Main loop
    fn main_loop(&mut self, terminal: &mut Terminal<impl Backend>) {
        use AppOp::*;
        loop {
            terminal.draw(|f| self.render(f)).unwrap();
            let event = event::read().unwrap();

            let op = self.mode.process_event(event, &self.canvas.cursor());

            match op {
                QuitApp => break,
                MakeShape(c, s) => self.canvas.add_shape(c, s),
                MoveCanvasCursor(d) => self.canvas.move_cursor(d),
                SetCanvasCursor(c) => self.canvas.set_cursor(c),
                Nop => {}
            }
        }
    }

    /// Run the application.
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
