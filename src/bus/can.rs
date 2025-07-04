use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct CanMessage {
    id: u32,
    name: String,
    alt_names: Vec<String>,
    length: u32,
    comment: Vec<String>,
    periodicity: u32,
}

pub fn load_message(path_to_file: &String) -> Result<(), serde_yaml::Error> {
    let file_content = fs::read_to_string(path_to_file)
        .expect("Unable to read the file.");

    let can_message: CanMessage = serde_yaml::from_str(&file_content)?;
    println!("{:?}", can_message);
    Ok(())
}

pub fn save_message(file_name: &String) -> Result<(), serde_yaml::Error> {
    let can_message = CanMessage {
        id: 0x208,
        name: "GENERIC_ENGINE_DATA".to_string(),
        alt_names: Vec::new(),
        length: 8,
        comment: Vec::new(),
        periodicity: 10,
    };
    let yaml = serde_yaml::to_string(&can_message)?;
    fs::write(file_name, yaml)
        .expect("Failed to write file.");
    // println!("{}", yaml);
    Ok(())
}
