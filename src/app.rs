mod canvas;
mod cmd_line;
mod mode;
mod shape;

use self::{canvas::CanvasHandler, mode::ModeHandler, shape::Shape};
use crate::util::{Coord, Size};
use canvas::ShapeIdSet;
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
    DeleteShapes(ShapeIdSet),
    MoveShapesAndCanvasCursor(ShapeIdSet, crate::util::Direction),
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

    fn render(&mut self, f: &mut Frame) {
        let canvas_area = Constraint::Length(f.area().height - 1);
        let cmd_line_area = Constraint::Length(1);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([canvas_area, cmd_line_area])
            .split(f.area());

        let canvas_area = &layout[0];
        let cmd_line_area = &layout[1];

        // Render canvas
        let canvas_size = Size::new(canvas_area.width, canvas_area.height);
        let additional_shapes = self
            .mode
            .get()
            .additinal_canvas_shapes(self.canvas_handler.cursor_coord());

        let shapes_to_highlight = self.mode.get().shapes_to_highlight();

        self.canvas_handler.set_rendering_size(canvas_size);
        self.canvas_handler.set_additional_shapes(additional_shapes);
        self.canvas_handler
            .set_shapes_to_highlight(shapes_to_highlight);
        f.render_widget(&mut self.canvas_handler, *canvas_area);

        // Render command line
        let cmd_line = self.mode.get().cmd_line();
        f.render_widget(cmd_line, *cmd_line_area);
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
                DeleteShapes(ids) => self.canvas_handler.delte_shapes(&ids),
                MoveShapesAndCanvasCursor(ids, dir) => {
                    self.canvas_handler.move_cursor(dir);
                    self.canvas_handler.move_shapes(&ids, dir);
                }
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
