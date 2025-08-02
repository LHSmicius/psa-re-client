use std::{fs, io::Write};
use yaml_rust2::{yaml, Yaml, YamlEmitter, YamlLoader};

pub struct Config {
    pub database_dir: String,
    pub show_messages: bool,
}

impl Config {
    fn load_config_str(yaml_str: &str) -> Config {
        let docs = YamlLoader::load_from_str(yaml_str)
                .expect("Failed to parse YAML content of configuration file.");
        let doc = &docs[0];
        
        let mut config = Config {
            database_dir: String::new(),
            show_messages: false,
        };

        if let Yaml::Hash(hash) = doc {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "database_dir" => {
                            if let Yaml::String(v) = value {
                                config.database_dir = v.clone();
                            } else {
                                println!("[WARNING] Wrong type for \"database_dir\".");
                            }
                        }
                        "show_messages" => {
                            if let Yaml::Boolean(v) = value {
                                config.show_messages = v.clone();
                            } else {
                                println!("[WARNING] Wrong type for \"show_messages\".");
                            }
                        }
                        _ => {
                            println!("[WARNING] Unknown configuration parameter \"{}\".", k);
                        }
                    }
                }
            }
        }
        config
    }

    fn save_config(file_path: &str, config: Config) -> Config {
        let mut hash = yaml::Hash::new();
        hash.insert(Yaml::String("database_dir".into()), Yaml::String(config.database_dir.clone()));
        hash.insert(Yaml::String("show_messages".into()), Yaml::Boolean(config.show_messages.clone()));

        let yaml_doc = Yaml::Hash(hash);
        let mut file_str = String::new();
        YamlEmitter::new(&mut file_str).dump(&yaml_doc).expect("Failed to emit YAML.");

        let mut new_config_file = fs::File::create(file_path).unwrap();
        new_config_file.write_all(file_str.as_bytes()).expect("Failed to write to file.");

        config
    }

    pub fn load_config_file(file_path: &str) -> Config {
        if fs::exists(file_path).unwrap() {
            print!("Loading configuration... ");
            let yaml_content = fs::read_to_string(file_path)
                    .expect("Unable to open configuration file.");
            let config = Self::load_config_str(&yaml_content);
            println!("Done.");
            config
        } else {
            let config = Config {
                database_dir: "../PSA-RE/buses/AEE2004.full/HS.IS/".to_string(),
                show_messages: false,
            };

            print!("Config file not found. Creating default... ");
            let config = Self::save_config(file_path, config);
            println!("Done.");
            config
        }
        
    }
}