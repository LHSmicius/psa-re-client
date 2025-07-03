use crate::bus::can;

pub mod bus;

fn main() {
    println!("PSA-RE-CLIENT");
    _ = can::read_message_structure();
    _ = can::save_message_structure();
}
