#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

mod http;

extern crate ctrlc;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}

    // To a graceful shutdown here
}
