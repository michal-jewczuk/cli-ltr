mod terminal;

mod screens;
mod ui;

use std::io;


fn main() -> Result<(), io::Error> {
    terminal::run()
}
