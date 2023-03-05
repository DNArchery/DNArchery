extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod api;
mod core;
mod plug;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    std::thread::spawn(|| api::server::spin());

    // Spin up integrated GUI
    crate::plug::gui::spin_gui_window();
}
