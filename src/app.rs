use crate::{
    canvas::{CanvasHandler, RenderState},
    controller::mode::normal::NormalMode,
    controller::mode::Mode,
    util::Size,
};
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

/// The application
pub struct App {
    canvas_handler: CanvasHandler,
    mode: Box<dyn Mode>,
}

impl App {
    pub fn new() -> Self {
        App {
            canvas_handler: CanvasHandler::default(),
            mode: Box::new(NormalMode),
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
        let mut state = RenderState::new(self.mode.as_ref(), canvas_size);
        f.render_stateful_widget(&mut self.canvas_handler, chunks1[0], &mut state);
        f.render_widget(&self.mode, chunks1[1]);
    }

    /// Main loop
    fn main_loop(&mut self, terminal: &mut Terminal<impl Backend>) {
        use crate::controller::AppOp::*;
        loop {
            terminal.draw(|f| self.render(f)).unwrap();
            let event = event::read().unwrap();
            let cursor_coord = self.canvas_handler.canvas.cursor();
            let op;
            let current_mode = std::mem::take(&mut self.mode);
            (self.mode, op) = current_mode.next(event, cursor_coord);

            match op {
                QuitApp => break,
                MakeShape(c, s) => self.canvas_handler.canvas.add_shape(c, s),
                MoveCanvasCursor(d) => self.canvas_handler.canvas.move_cursor(d),
                SetCanvasCursor(c) => self.canvas_handler.canvas.set_cursor(c),
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
