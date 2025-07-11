use crate::bus::can::{self, Translation};
use std::{env, fmt::Debug, process};

pub mod bus;

fn print_usage(program_name: &str) {
    println!("Usage: {} <yaml-file>", program_name);
    println!("  yaml-file: Path to the YAML file to load");
    println!("");
    println!("Example:");
    println!("  {} can_message.yaml", program_name);
}

fn print_optional_field<T: Debug>(name: &str, field: &Option<T>) {
    if let Some(val) = field {
        println!("{}: {:?}", name, val);
    }
}

fn print_translation(mut name: String, field: &Option<Translation>) {
    if let Some(translation) = field {
        name.push_str(" EN");
        print_optional_field(&name, &translation.en);
        name = format!("{}FR", &name[..name.len() - 2]);
        print_optional_field(&name, &translation.fr);
        name = format!("{}DE", &name[..name.len() - 2]);
        print_optional_field(&name, &translation.de);
    }
}

fn print_values(name: String, value_vec: &Vec<(i64, Option<Translation>)>) {
    for val in value_vec {
        let val_str = format!(" = 0x{:X}", val.0);
        let line = name.clone() + &val_str;
        print_translation(line, &val.1);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Error: Incorrect number of arguments");
        print_usage(&args[0]);
        process::exit(1);
    }
    
    let yaml_file = &args[1];
    
    // Check if help is requested
    if yaml_file == "--help" || yaml_file == "-h" {
        print_usage(&args[0]);
        return Ok(());
    }

    println!("PSA-RE-CLIENT opening file {}.", yaml_file);
    let can_message = can::CanMessage::from_yaml_file(&yaml_file)?;
    
    println!("Loaded CAN Message:");
    print_optional_field("ID", &can_message.id);
    print_optional_field("Name", &can_message.name);
    print_optional_field("Alternative names", &can_message.alt_names);
    print_optional_field("Length", &can_message.length);
    print_optional_field("Type", &can_message.bus_type);
    print_optional_field("Periodicity", &can_message.periodicity);
    println!("Senders: {:?}", can_message.senders);
    println!("Receivers: {:?}", can_message.receivers);
    print_translation("Comment".to_string(), &can_message.comment);

    println!("Signals:");
    for (signal_name, signal) in &can_message.signals {
        println!("  {}", signal_name);
        print_optional_field("    Bits", &signal.bits);
        print_optional_field("    Type", &signal.data_type);
        print_optional_field("    Factor", &signal.factor);
        print_optional_field("    Offset", &signal.offset);
        print_optional_field("    Min", &signal.min);
        print_optional_field("    Max", &signal.max);
        print_translation("    Comment".to_string(), &signal.comment);
        print_values("    Value".to_string(), &signal.values);
    }
    
    Ok(())
}
