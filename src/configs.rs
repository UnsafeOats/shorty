use toml::from_str;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env::var;
use std::io::Write;
use std::path::PathBuf;
use dirs::home_dir;
use std::fs;
use anyhow::Result;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Configs {
    pub settings: Settings,
    pub shortcuts: HashMap<String, String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Settings {
    pub keypress: String,
}

impl Configs {
    pub fn new() -> Configs {
        let mut home = home_dir().unwrap();
        home.push(".config");
        let config_file = var("SHORTY_CONFIG_FILE");
        let config_path = match config_file {
            Ok(path) => PathBuf::from(&path),
            Err(_) => {
                let mut default_config_path = home.clone();
                default_config_path.push(".shorty.toml");
                default_config_path
            },
        };
        if !config_path.exists() {
            fs::create_dir_all(&config_path.parent().unwrap_or(&home)).expect("[error] Could not create config directory.");
            let mut new_config_file = fs::File::create(&config_path).expect("[error] Could not create config file.");
            new_config_file.write_all(include_str!("resources/default.toml").as_bytes()).expect("[error] Could not write to config file.");
        };
        let toml_string = fs::read_to_string(config_path).expect("[error] Could not read from config file.");
        let configs: Configs = from_str(&toml_string).expect("[error] Could not parse toml in config file.");
        configs
    }
}
