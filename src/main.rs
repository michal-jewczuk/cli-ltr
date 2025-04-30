mod terminal;
mod app;

mod screens;
mod ui;
mod models;
mod service;

use service::testservice;
use rusqlite::Connection;

#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

fn main() -> Result<(), std::io::Error> {
    terminal::run()
}

