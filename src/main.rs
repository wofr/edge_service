#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

mod http;
mod udp;
extern crate ctrlc;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use structopt::StructOpt;
use http::start_http_endpoint;


// Command line arguments struct
#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(short = "h", long = "httpPort")]
    pub port_http: Option<u16>,
}

fn main() {

    // Parse command line arguments
    let args = Cli::from_args();

    // Start Http Endpoint

    start_http_endpoint(&args.port_http);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}

    // To a graceful shutdown here
}
