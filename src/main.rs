use shrtcut::configs;
use shrtcut::gui;
use std::env;

fn spawn_app(configs: configs::Configs) {
    match gui::GuiApp::new(configs).spawn() {
        Ok(_) => (),
        Err(e) => println!("[error] {:?}", e),
    };
}

fn main() {
    let configs = configs::Configs::new();
    match env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "--help" | "-h" => {
                println!("{}", include_str!("resources/help.txt"));
            },
            "--version" | "-v" => {
                println!("shrtcut::v.{}", env!("CARGO_PKG_VERSION"));
            },
            "--grab" | "-g" => match env::args().nth(2) {
                Some(choice) => {
                    match configs.copy_to_clipboard(choice) {
                        Ok(_) => (),
                        Err(e) => println!("{:?}", e),
                    };
                },
                None => println!("No shortcut provided. Try --help for more information."),
            },
            "--add" | "-a" => match env::args().nth(2) {
                Some(choice) => {
                    match configs.create_shortcut_from_clipboard(choice) {
                        Ok(_) => (),
                        Err(e) => println!("{:?}", e),
                    };
                },
                None => println!("No shortcut provided. Try --help for more information."),
            },
            "--configs" | "-c" => {
                configs.print_configs();
            },
            "--select" | "-s" => spawn_app(configs),
            "--list" | "-l" => {
                configs.print_shortcuts();
            },
            _ => println!("Invalid argument. Try --help for more information."),
        },
        None => spawn_app(configs),
    }
}
