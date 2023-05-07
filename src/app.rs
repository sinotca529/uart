use crate::{canvas::Canvas, controller::mode::Mode};
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

pub struct App {
    canvas: Canvas,
    mode: Mode,
}

impl App {
    pub fn new() -> Self {
        App {
            canvas: Canvas::new(),
            mode: Mode::new(),
        }
    }

    fn render(&self, f: &mut Frame<impl Backend>) {
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

        f.render_stateful_widget(&self.canvas, chunks1[0], &mut &self.mode);
        f.render_widget(&self.mode, chunks1[1]);
    }

    fn main_loop(&mut self, terminal: &mut Terminal<impl Backend>) {
        use crate::controller::AppOp::*;
        loop {
            terminal.draw(|f| self.render(f)).unwrap();
            let op = self
                .mode
                .trans(event::read().unwrap(), self.canvas.cursor());
            match op {
                QuitApp => break,
                MakeShape(c, s) => self.canvas.add_shape(c, s),
                MoveCanvasCursor(d) => self.canvas.move_cursor(d),
                SetCanvasCursor(c) => self.canvas.set_cursor(c),
                Nop => {}
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
