use crate::{bus::can::{self, CanMessage}, config::Config};
use std::{env, fmt::Debug, fs, process};

pub mod config;
pub mod bus;

fn print_usage(program_name: &str) {
    println!("Usage: {} [OPTION]... [YAML-DIR]...", program_name);
    println!("  YAML-DIR: Directory where .yaml or .yml files are located");
    println!("OPTIONS:");
    println!("  -h, --help      show this usage explanation");
    println!("  -s, --show      show loaded CAN messages");
    println!("");
    println!("Example:");
    println!("  {} -s ../PSA-RE/buses/AEE2004.full/LS.CONF/", program_name);
}

fn print_optional_field<T: Debug>(name: &str, field: &Option<T>) {
    if let Some(val) = field {
        println!("{}: {:?}", name, val);
    }
}

fn print_translation(mut name: String, field: &Option<can::Translation>) {
    if let Some(translation) = field {
        name.push_str(" EN");
        print_optional_field(&name, &translation.en);
        name = format!("{}FR", &name[..name.len() - 2]);
        print_optional_field(&name, &translation.fr);
        name = format!("{}DE", &name[..name.len() - 2]);
        print_optional_field(&name, &translation.de);
    }
}

fn print_values(name: String, value_vec: &Vec<(i64, Option<can::Translation>)>) {
    for val in value_vec {
        let val_str = format!(" = 0x{:X}", val.0);
        let line = name.clone() + &val_str;
        print_translation(line, &val.1);
    }
}

fn print_can_message(message: &can::CanMessage) {
    println!("Loaded CAN Message:");
    print_optional_field("ID", &message.id);
    print_optional_field("Name", &message.name);
    print_optional_field("Alternative names", &message.alt_names);
    print_optional_field("Length", &message.length);
    print_optional_field("Type", &message.bus_type);
    print_optional_field("Periodicity", &message.periodicity);
    println!("Senders: {:?}", message.senders);
    println!("Receivers: {:?}", message.receivers);
    print_translation("Comment".to_string(), &message.comment);

    println!("Signals:");
    for (signal_name, signal) in &message.signals {
        println!("  {}", signal_name);
        print_optional_field("    Alternative names", &signal.alt_names);
        print_optional_field("    Bits", &signal.bits);
        print_optional_field("    Type", &signal.data_type);
        print_optional_field("    Signed", &signal.signed);
        print_optional_field("    Factor", &signal.factor);
        print_optional_field("    Offset", &signal.offset);
        print_optional_field("    Min", &signal.min);
        print_optional_field("    Max", &signal.max);
        print_optional_field("    Units", &signal.units);
        print_translation("    Comment".to_string(), &signal.comment);
        print_values("    Value".to_string(), &signal.values);
        print_optional_field("    Unused", &signal.unused);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut config: config::Config = Config::load_config_file("config.yaml");

    let program_name = &args[0];
    let mut can_messages: Vec<CanMessage> = Vec::new();

    for i in 1..args.len() {
        if args[i].starts_with("-") { // OPTIONS
            if args[i] == "--help" || args[i] == "-h" {
                print_usage(&program_name);
                process::exit(0);
            } else
            if args[i] == "--show" || args[i] == "-s" {
                config.show_messages = true;
            }
        } else { // DB DIR
            config.database_dir = args[i].clone();
        }        
    }

    for file_in_path in fs::read_dir(config.database_dir)? {
        let file_path = file_in_path?;
        let f_path = file_path.path();

        if f_path.is_file() {
            if let Some(f_ext) = f_path.extension() {
                if f_ext == "yml" || f_ext == "yaml" {
                    if let Some(file_path_str) = f_path.to_str() {
                        println!("PSA-RE-CLIENT opening file {}.", file_path_str);
                        let can_message = can::CanMessage::from_yaml_file(&file_path_str)?;
                        if config.show_messages {
                            print_can_message(&can_message);
                        }
                        can_messages.push(can_message);
                    }
                }
            }
        }
    }
    Ok(())
}
