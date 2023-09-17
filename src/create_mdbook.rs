//mdbook
use crate::message_alert;

use mdbook::config::Config;
use mdbook::errors::Error;
use mdbook::MDBook;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

pub fn create_mdbook() {
    let _ = fs::create_dir("./mdBook_html_files/");
    let root_dir = "./mdBook_html_files/";
    let mut cfg = Config::default();

    cfg.book.title = Some("TEST".to_string());

    MDBook::init(root_dir)
        .create_gitignore(true)
        .with_config(cfg)
        .build()
        .expect("Book generation failed");

    match fs::remove_file("./mdBook_html_files/src/SUMMARY.md") {
        Ok(_) => println!("## OK"),
        Err(err) => println!(" ## failed : {}", err),
    }
}

pub fn build_mdbook() -> Result<(), Error> {
    let md = MDBook::load("./mdBook_html_files/");
    match md {
        Ok(book) => match book.build() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
