mod canvas;
mod mode;
mod shape;

use self::{
    canvas::{CanvasHandler, RenderState},
    mode::ModeHandler,
    shape::Shape,
};
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
pub struct App {
    canvas_handler: CanvasHandler,
    mode: ModeHandler,
}

impl App {
    pub fn new() -> Self {
        App {
            canvas_handler: CanvasHandler::default(),
            mode: ModeHandler::default(),
        }
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
        let mut state = RenderState::new(self.mode.get(), canvas_size);
        f.render_stateful_widget(&mut self.canvas_handler, chunks1[0], &mut state);
        f.render_widget(self.mode.get(), chunks1[1]);
    }

    /// Main loop
    fn main_loop(&mut self, terminal: &mut Terminal<impl Backend>) {
        use AppOp::*;
        loop {
            terminal.draw(|f| self.render(f)).unwrap();
            let event = event::read().unwrap();

            let canvas = self.canvas_handler.canvas_mut();
            let canvas_cursor = canvas.cursor_mut();
            let op = self.mode.process_event(event, canvas_cursor);

            match op {
                QuitApp => break,
                MakeShape(c, s) => canvas.add_shape(c, s),
                MoveCanvasCursor(d) => canvas_cursor.move_next(d),
                SetCanvasCursor(c) => canvas_cursor.move_to(c),
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
