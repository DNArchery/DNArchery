extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod api;
mod core;

use api::server;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    // The interface for DNArchery is a web server with endpoints utilized by the UI
    server::spin()
        .expect("Failed to spin up API server");
}
