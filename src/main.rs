mod app;
mod big_font;
mod config;
mod notification;
mod timer;
mod ui;

use clap::Parser;

use app::App;
use config::Config;

fn main() -> std::io::Result<()> {
    let config = Config::parse();
    let mut app = App::new(config);

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal);
    ratatui::restore();

    result
}
