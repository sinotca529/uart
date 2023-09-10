mod canvas;
mod cmd_line;
mod mode;
mod shape;

use self::{canvas::CanvasHandler, mode::ModeHandler, shape::Shape};
use crate::util::{Coord, Size};
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

pub enum AppOp {
    MakeShape(Coord, Box<dyn Shape>),
    MoveCanvasCursor(crate::util::Direction),
    SetCanvasCursor(Coord),
    ToggleShapeSelect,
    DeleteSelectedShapes,
    MoveSlectedShapes(crate::util::Direction),
    UnselectAllShape,
    QuitApp,
    Nop,
}

/// The application
#[derive(Default)]
pub struct App {
    canvas_handler: CanvasHandler,
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

        // Render canvas
        let canvas_size = Size::new(chunks1[0].width, chunks1[0].height);
        let additional_shapes = self
            .mode
            .get()
            .additinal_canvas_shapes(self.canvas_handler.cursor_coord());

        self.canvas_handler.set_rendering_size(canvas_size);
        self.canvas_handler.set_additional_shapes(additional_shapes);
        f.render_widget(&mut self.canvas_handler, chunks1[0]);

        // Render command line
        let cmd_line = self.mode.get().cmd_line();
        f.render_widget(cmd_line, chunks1[1]);
    }

    /// Main loop
    fn main_loop(&mut self, terminal: &mut Terminal<impl Backend>) {
        use AppOp::*;
        loop {
            terminal.draw(|f| self.render(f)).unwrap();
            let event = event::read().unwrap();

            let op = self.mode.process_event(event, &self.canvas_handler);

            match op {
                QuitApp => break,
                MakeShape(c, s) => self.canvas_handler.add_shape(c, s),
                MoveCanvasCursor(d) => self.canvas_handler.move_cursor(d),
                SetCanvasCursor(c) => self.canvas_handler.set_cursor(c),
                ToggleShapeSelect => self.canvas_handler.toggle_select(),
                DeleteSelectedShapes => self.canvas_handler.delete_selected_shapes(),
                MoveSlectedShapes(d) => self.canvas_handler.move_selected_shapes(d),
                UnselectAllShape => self.canvas_handler.unselect_all_shape(),
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
