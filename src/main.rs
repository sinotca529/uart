use app::{config::Config, App};
mod app;
mod util;

fn main() {
    let config = Config::load().unwrap();
    println!("{:?}", config);
    App::new().run();
}
