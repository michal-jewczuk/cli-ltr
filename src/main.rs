mod terminal;
mod app;

mod screens;
mod ui;
mod models;
mod service;

use std::io;


fn main() -> Result<(), io::Error> {
    terminal::run()
}
