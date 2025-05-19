mod terminal;
mod app;

mod screens;
mod ui;
mod models;
mod service;
use service::ioservice;

extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

fn main() -> Result<(), std::io::Error> {
    //terminal::run()
    ioservice::import_test_files("pl");
    Ok(())
}

