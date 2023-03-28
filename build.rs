use std::env::var;
use std::path::PathBuf;
use std::include_str;
use std::fs;
use std::io::Write;
use dirs::home_dir;

fn generate_config() {
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
    if !config_path.exists() {
            fs::create_dir_all(&config_path.parent().unwrap_or(&home_dir().expect("[error] Cannot find home directory.")))
                .expect("[error] Could not create config directory.");
            let mut new_config_file =
                fs::File::create(&config_path).expect("[error] Could not create config file.");
            new_config_file
                .write_all(include_str!("src/resources/default.toml").as_bytes())
                .expect("[error] Could not write to config file.");
    };
}

fn main() {
    println!("cargo:rerun-if-env-changed=SHRTCUT_CONFIG_FILE");
    generate_config();
}
