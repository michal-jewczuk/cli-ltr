mod terminal;
mod app;

mod screens;
mod ui;

use std::io;


fn main() -> Result<(), io::Error> {
    terminal::run()
}
