use app::{config::Config, App};
mod app;
mod util;

fn main() {
    let config = Config::load().unwrap();
    App::new(&config.into()).run();
}
