use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]

struct CanMessage {
    id: u32,
    name: String,
    length: u32,
    periodicity: u32,
}

fn main() -> Result<(), serde_yaml::Error> {
    println!("PSA-RE-CLIENT");
    let can_message = CanMessage {
        id: 0x208,
        name: "GENERIC_ENGINE_DATA".to_string(),
        length: 8,
        periodicity: 10,
    };
    
    let yaml = serde_yaml::to_string(&can_message)?;
    println!("{}", yaml);

    let parsed_message: CanMessage = serde_yaml::from_str(&yaml)?;
    println!("{:?}", parsed_message);

    Ok(())
}
