use crate::bus::can;
use std::{env, process};

pub mod bus;

fn print_usage(program_name: &str) {
    println!("Usage: {} <yaml-file>", program_name);
    println!("  yaml-file: Path to the YAML file to load");
    println!("");
    println!("Example:");
    println!("  {} can_message.yaml", program_name);
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
    println!("ID: {:?}", can_message.id);
    println!("Name: {:?}", can_message.name);
    println!("Length: {:?}", can_message.length);
    println!("Type: {:?}", can_message.bus_type);
    println!("Periodicity: {:?}", can_message.periodicity);
    println!("Senders: {:?}", can_message.senders);
    println!("Receivers: {:?}", can_message.receivers);
    
    if let Some(comment) = &can_message.comment {
        println!("Comment (EN): {:?}", comment.en);
        println!("Comment (FR): {:?}", comment.fr);
        println!("Comment (DE): {:?}", comment.de);
    }

    println!("\nSignals:");
    for (signal_name, signal) in &can_message.signals {
        println!("  {}: {:?}", signal_name, signal);
    }
    
    Ok(())
}
