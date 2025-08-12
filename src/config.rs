use log::{debug, warn};
use std::{fs, io::Write};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader, yaml};

pub struct Config {
    pub database_dir: String,
    pub default_lang: String,
}

impl Config {
    fn get_default_config() -> Config {
        Config {
            database_dir: String::from("../PSA-RE/buses/AEE2004.full/HS.IS/"),
            default_lang: String::from("en"),
        }
    }

    fn load_config_str(yaml_str: &str) -> Config {
        let docs = YamlLoader::load_from_str(yaml_str)
            .expect("Failed to parse YAML content of configuration file.");
        let doc = &docs[0];

        let mut config = Self::get_default_config();

        if let Yaml::Hash(hash) = doc {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "database_dir" => {
                            if let Yaml::String(v) = value {
                                config.database_dir = v.clone();
                            } else {
                                warn!("[WARNING] Wrong type for \"database_dir\".");
                            }
                        }
                        "default_lang" => {
                            if let Yaml::String(v) = value {
                                config.default_lang = v.clone();
                            } else {
                                warn!("[WARNING] Wrong type for \"default_lang\".");
                            }
                        }
                        _ => {
                            warn!("[WARNING] Unknown configuration parameter \"{}\".", k);
                        }
                    }
                }
            }
        }
        config
    }

    fn save_config(file_path: &str, config: Config) -> Config {
        let mut hash = yaml::Hash::new();
        hash.insert(
            Yaml::String("database_dir".into()),
            Yaml::String(config.database_dir.clone()),
        );
        hash.insert(
            Yaml::String("default_lang".into()),
            Yaml::String(config.default_lang.clone()),
        );

        let yaml_doc = Yaml::Hash(hash);
        let mut file_str = String::new();
        YamlEmitter::new(&mut file_str)
            .dump(&yaml_doc)
            .expect("Failed to emit YAML.");

        let mut new_config_file = fs::File::create(file_path).unwrap();
        new_config_file
            .write_all(file_str.as_bytes())
            .expect("Failed to write to file.");
        config
    }

    pub fn load_config_file(file_path: &str) -> Config {
        if fs::exists(file_path).unwrap() {
            debug!("Loading configuration.");
            let yaml_content =
                fs::read_to_string(file_path).expect("Unable to open configuration file.");
            let config = Self::load_config_str(&yaml_content);
            config
        } else {
            let config = Self::get_default_config();
            debug!("Config file not found. Creating default one.");
            let config = Self::save_config(file_path, config);
            config
        }
    }
}
