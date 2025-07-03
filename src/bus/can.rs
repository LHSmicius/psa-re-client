use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CanMessage {
    id: u32,
    name: String,
    length: u32,
    periodicity: u32,
}

pub fn read_message_structure() -> Result<(), serde_yaml::Error> {
    let can_message = CanMessage {
        id: 0x308,
        name: "ENGINE_DATA".to_string(),
        length: 7,
        periodicity: 100,
    };
    let yaml = serde_yaml::to_string(&can_message)?;
    let parsed_message: CanMessage = serde_yaml::from_str(&yaml)?;
    println!("{:?}", parsed_message);
    Ok(())
}

pub fn save_message_structure() -> Result<(), serde_yaml::Error> {
    let can_message = CanMessage {
        id: 0x208,
        name: "GENERIC_ENGINE_DATA".to_string(),
        length: 8,
        periodicity: 10,
    };
    let yaml = serde_yaml::to_string(&can_message)?;
    println!("{}", yaml);
    Ok(())
}
