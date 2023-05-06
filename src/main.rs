use controller::{
    command_stream::CommandStream,
    Controller,
};
use model::Model;
use view::View;

mod controller;
mod model;
mod util;
mod view;

fn main() {
    let model = Model::new();
    let ctrl = Controller::new(model);
    let cmd_stream = CommandStream::new();
    let mut view = View::new(ctrl, cmd_stream);
    view.run();
}
