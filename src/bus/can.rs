use std::collections::HashMap;
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
    pub bits: Option<String>,
    pub data_type: Option<String>,
    pub factor: Option<f64>,
    pub offset: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub comment: Option<Translation>,
}

#[derive(Debug, Clone)]
pub struct CanMessage {
    pub id: Option<String>,
    pub name: Option<String>,
    pub length: Option<i64>,
    pub comment: Option<Translation>,
    pub bus_type: Option<String>,
    pub periodicity: Option<i64>,
    pub senders: Vec<String>,
    pub receivers: Vec<String>,
    pub signals: HashMap<String, Signal>,
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
                        _ => {}
                    }
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
            bits: None,
            data_type: None,
            factor: None,
            offset: None,
            min: None,
            max: None,
            comment: None,
        };

        if let Yaml::Hash(hash) = yaml {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "bits" => {
                            if let Yaml::String(v) = value {
                                signal.bits = Some(v.clone());
                            }
                        }
                        "type" => {
                            if let Yaml::String(v) = value {
                                signal.data_type = Some(v.clone());
                            }
                        }
                        "factor" => {
                            signal.factor = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => None,
                            };
                        }
                        "offset" => {
                            signal.offset = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => None,
                            };
                        }
                        "min" => {
                            signal.min = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => None,
                            };
                        }
                        "max" => {
                            signal.max = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => None,
                            };
                        }
                        "comment" => {
                            signal.comment = Translation::from_yaml(value);
                        }
                        _ => {}
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
            length: None,
            comment: None,
            bus_type: None,
            periodicity: None,
            senders: Vec::new(),
            receivers: Vec::new(),
            signals: HashMap::new(),
        };

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
                            }
                        }
                        "length" => {
                            if let Yaml::Integer(v) = value {
                                message.length = Some(*v);
                            }
                        }
                        "comment" => {
                            message.comment = Translation::from_yaml(value);
                        }
                        "type" => {
                            if let Yaml::String(v) = value {
                                message.bus_type = Some(v.clone());
                            }
                        }
                        "periodicity" => {
                            if let Yaml::Integer(v) = value {
                                message.periodicity = Some(*v);
                            }
                        }
                        "senders" => {
                            if let Yaml::Array(arr) = value {
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        message.senders.push(s.clone());
                                    }
                                }
                            }
                        }
                        "receivers" => {
                            if let Yaml::Array(arr) = value {
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        message.receivers.push(s.clone());
                                    }
                                }
                            }
                        }
                        "signals" => {
                            if let Yaml::Hash(signals_hash) = value {
                                for (signal_key, signal_value) in signals_hash {
                                    if let Yaml::String(signal_name) = signal_key {
                                        let signal = Signal::from_yaml(signal_value);
                                        message.signals.insert(signal_name.clone(), signal);
                                    }
                                }
                            }
                        }
                        _ => {}
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
