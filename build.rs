use std::env::var;
use dir::home_dir;

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
}

fn main() {
    println!("cargo:rerun-if-env-changed=SHRTCUT_CONFIG_FILE");
    let _ = Configs::new();
}
