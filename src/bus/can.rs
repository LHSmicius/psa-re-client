use std::fs;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Debug, Clone)]
pub struct Translation {
    pub en: Option<String>,
    pub fr: Option<String>,
    pub de: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub alt_names: Option<Vec<String>>,
    pub bits: Option<String>,
    pub data_type: Option<String>,
    pub signed: Option<bool>,
    pub factor: Option<f64>,
    pub offset: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub units: Option<String>,
    pub comment: Option<Translation>,
    pub values: Vec<(i64, Option<Translation>)>,
    pub unused: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct CanMessage {
    pub id: Option<String>,
    pub name: Option<String>,
    pub alt_names: Option<Vec<String>>,
    pub length: Option<i64>,
    pub comment: Option<Translation>,
    pub bus_type: Option<String>,
    pub periodicity: Option<i64>,
    pub senders: Vec<String>,
    pub receivers: Vec<String>,
    pub signals: Vec<(String, Signal)>,
}

impl Translation {
    fn from_yaml(yaml: &Yaml) -> Option<Translation> {
        if let Yaml::Hash(hash) = yaml {
            let mut translation = Translation {
                en: None,
                fr: None,
                de: None,
            };
            
            for (key, value) in hash {
                if let (Yaml::String(k), Yaml::String(v)) = (key, value) {
                    match k.as_str() {
                        "en" => translation.en = Some(v.clone()),
                        "fr" => translation.fr = Some(v.clone()),
                        "de" => translation.de = Some(v.clone()),
                        _ => {
                            println!("[WARNING] Unsupported language \"{}\".", k);
                        }
                    }
                } else {
                    println!("[WARNING] Wrong type for language translation.");
                }
            }
            
            Some(translation)
        } else {
            None
        }
    }
}

impl Signal {
    fn from_yaml(yaml: &Yaml) -> Signal {
        let mut signal = Signal {
            alt_names: None,
            bits: None,
            data_type: None,
            signed: None,
            factor: None,
            offset: None,
            min: None,
            max: None,
            units: None,
            comment: None,
            values: Vec::new(),
            unused: None,
        };

        if let Yaml::Hash(hash) = yaml {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "alt_names" => {
                            if let Yaml::Array(arr) = value {
                                let mut alt_names = Vec::new();
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        alt_names.push(s.clone());
                                    }
                                }
                                if !alt_names.is_empty() {
                                    signal.alt_names = Some(alt_names);
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"alt_names\".");
                            }
                        }
                        "bits" => {
                            if let Yaml::String(v) = value {
                                signal.bits = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"bits\".");
                            }
                        }
                        "type" => {
                            if let Yaml::String(v) = value {
                                signal.data_type = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"type\".");
                            }
                        }
                        "signed" => {
                            if let Yaml::Boolean(v) = value {
                                signal.signed = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"signed\".");
                            }
                        }
                        "factor" => {
                            signal.factor = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    println!("[WARNING] Wrong type for \"factor\".");
                                    None
                                }
                            };
                        }
                        "offset" => {
                            signal.offset = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    println!("[WARNING] Wrong type for \"offset\".");
                                    None
                                }
                            };
                        }
                        "min" => {
                            signal.min = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    println!("[WARNING] Wrong type for \"min\".");
                                    None
                                }
                            };
                        }
                        "max" => {
                            signal.max = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    println!("[WARNING] Wrong type for \"max\".");
                                    None
                                }
                            };
                        }
                        "units" => {
                            if let Yaml::String(v) = value {
                                signal.units = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"units\".");
                            }
                        }
                        "comment" => {
                            signal.comment = Translation::from_yaml(value);
                        }
                        "values" => {
                            if let Yaml::Hash(value_hash) = value {
                                for (value_key, value_val) in value_hash {
                                    if let Yaml::Integer(value_num) = value_key {
                                        let explanation = Translation::from_yaml(value_val);
                                        signal.values.push((value_num.clone(), explanation));
                                    } else {
                                        println!("[WARNING] Expected integer in field \"values\".");
                                    }
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"values\".");
                            }
                        }
                        "unused" => {
                            if let Yaml::Boolean(v) = value {
                                signal.unused = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"unused\".");
                            }
                        }
                        _ => {
                            println!("[WARNING] Unknown CAN signal parameter \"{}\".", k);
                        }
                    }
                }
            }
        }
        signal
    }
}

impl CanMessage {
    pub fn from_yaml_str(yaml_str: &str) -> Result<CanMessage, Box<dyn std::error::Error>> {
        let docs = YamlLoader::load_from_str(yaml_str)?;
        let doc = &docs[0];
        
        let mut message = CanMessage {
            id: None,
            name: None,
            alt_names: None,
            length: None,
            comment: None,
            bus_type: None,
            periodicity: None,
            senders: Vec::new(),
            receivers: Vec::new(),
            signals: Vec::new(),
        };

        println!("Loading CAN message header.");
        if let Yaml::Hash(hash) = doc {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "id" => {
                            message.id = match value {
                                Yaml::String(v) => Some(v.clone()),
                                Yaml::Integer(v) => Some(format!("0x{:X}", v)),
                                _ => None,
                            };
                        }
                        "name" => {
                            if let Yaml::String(v) = value {
                                message.name = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"name\".");
                            }
                        }
                        "alt_names" => {
                            if let Yaml::Array(arr) = value {
                                let mut alt_names = Vec::new();
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        alt_names.push(s.clone());
                                    }
                                }
                                if !alt_names.is_empty() {
                                    message.alt_names = Some(alt_names);
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"alt_names\".");
                            }
                        }
                        "length" => {
                            if let Yaml::Integer(v) = value {
                                message.length = Some(*v);
                            } else {
                                println!("[WARNING] Wrong type for \"length\".");
                            }
                        }
                        "comment" => {
                            message.comment = Translation::from_yaml(value);
                        }
                        "type" => {
                            if let Yaml::String(v) = value {
                                message.bus_type = Some(v.clone());
                            } else {
                                println!("[WARNING] Wrong type for \"type\".");
                            }
                        }
                        "periodicity" => {
                            if let Yaml::Integer(number) = value {
                                message.periodicity = Some(number.clone());
                            } else if let Yaml::String(text) = value {
                                if text.eq("trigger") {
                                    message.periodicity = Some(-1);
                                } else if text.ends_with(" ms") {
                                    let number: i64 = text.trim_end_matches(" ms").parse().unwrap();
                                    message.periodicity = Some(number);
                                } else if text.ends_with("ms") {
                                    let number: i64 = text.trim_end_matches("ms").parse().unwrap();
                                    message.periodicity = Some(number);
                                } else {
                                    println!("[WARNING] Unable to parse \"periodicity\".");
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"periodicity\".");
                            }
                        }
                        "senders" => {
                            if let Yaml::Array(arr) = value {
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        message.senders.push(s.clone());
                                    }
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"senders\".");
                            }
                        }
                        "receivers" => {
                            if let Yaml::Array(arr) = value {
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        message.receivers.push(s.clone());
                                    }
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"receivers\".");
                            }
                        }
                        "signals" => {
                            if let Yaml::Hash(signals_hash) = value {
                                for (signal_key, signal_value) in signals_hash {
                                    if let Yaml::String(signal_name) = signal_key {
                                        println!("Loading CAN signal: {}.", signal_name);
                                        let signal = Signal::from_yaml(signal_value);
                                        message.signals.push((signal_name.clone(), signal));
                                    }
                                }
                            } else {
                                println!("[WARNING] Wrong type for \"signals\".");
                            }
                        }
                        _ => {
                            println!("[WARNING] Unknown CAN message parameter \"{}\".", k);
                        }
                    }
                }
            }
        }
        Ok(message)
    }

    pub fn from_yaml_file(file_path: &str) -> Result<CanMessage, Box<dyn std::error::Error>> {
        let yaml_content = fs::read_to_string(file_path)?;
        Self::from_yaml_str(&yaml_content)
    }
}
