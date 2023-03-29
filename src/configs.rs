use anyhow::{bail, Result};
use dirs::home_dir;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env::var;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use toml::from_str;
use glcp::GlobalClip;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Configs {
    pub settings: Settings,
    pub shortcuts: HashMap<String, String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Settings {
    pub width: i32,
    pub height: i32,
    pub default: Option<String>,
    pub env_annotation: Option<String>,
    pub add_to_clipboard: Option<bool>,
    pub print_to_stdout: Option<bool>,
}

impl Configs {
    pub fn new() -> Configs {
        let config_path = Self::get_config_path();
        if !config_path.exists() {
            fs::create_dir_all(&config_path.parent().unwrap_or(&home_dir().expect("[error] Cannot find home directory.")))
                .expect("[error] Could not create config directory.");
            let mut new_config_file =
                fs::File::create(&config_path).expect("[error] Could not create config file.");
            new_config_file
                .write_all(include_str!("resources/default.toml").as_bytes())
                .expect("[error] Could not write to config file.");
        };
        let toml_string =
            fs::read_to_string(config_path).expect("[error] Could not read from config file.");
        let configs: Configs =
            from_str(&toml_string).expect("[error] Could not parse toml in config file.");
        configs
    }

    fn get_config_path() -> PathBuf {
        let mut home = home_dir().unwrap();
        home.push(".config");
        let config_file = var("SHRTCUT_CONFIG_FILE");
        let config_path = match config_file {
            Ok(path) => PathBuf::from(&path),
            Err(_) => {
                let mut default_config_path = home.clone();
                default_config_path.push(".shrtcut.toml");
                default_config_path
            }
        };
        config_path
    }

    fn add_or_print_shortcut(&self, shortcut: String) -> Result<()> {
        let env_annotation = self.settings.env_annotation.clone().unwrap_or("$".to_string());
        let resolved_shortcut = if shortcut.starts_with(&env_annotation) {
            var(&shortcut[env_annotation.len()..])?
        } else {
            shortcut.to_string()
        };
        if self.settings.add_to_clipboard.unwrap_or(true) {
            GlobalClip::set(&resolved_shortcut)?;
        }
        if self.settings.print_to_stdout.unwrap_or(true) {
            println!("{}", &resolved_shortcut);
        }
        Ok(())
    }

    pub fn use_shortcut(&self, choice: String) -> Result<()> {
        let shorty = self.shortcuts.get(&choice);
        match shorty {
            Some(s) => self.add_or_print_shortcut(s.to_string())?,
            None => bail!("[error] Could not read shortcut."),
        };
        Ok(())
    }

    pub fn create_shortcut_from_clipboard(&self, name: String) -> Result<()> {
        let clipboard = GlobalClip::get()?;
        let command = format!("{}=\"{}\"", name, clipboard);
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(Self::get_config_path())?;
        writeln!(config_file, "{}", command)?;
        Ok(())
    }

    pub fn print_configs(&self) {
        let config_path = Self::get_config_path();
        println!("{}", config_path.to_str().unwrap_or(""));
    }

    pub fn print_shortcuts(&self) {
        let max_shortcut_length = self
            .shortcuts
            .keys()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);
        for (key, value) in &self.shortcuts {
            println!("{:0max_shortcut_length$} -> {}", key, value, max_shortcut_length = max_shortcut_length);
        }
    }
}
