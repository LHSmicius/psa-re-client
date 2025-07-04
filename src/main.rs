use crate::bus::can;

pub mod bus;

fn main() {
    println!("PSA-RE-CLIENT");
    let file = String::from("305.yml");
    _ = can::load_message(&file);
    _ = can::save_message(&"208.yml".to_string());
}
