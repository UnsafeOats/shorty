use crate::configs;
use fltk::{prelude::*, *};

struct GuiApp {
    window: window::Window,
    menu: menu::Choice,
    configs: configs::Configs,
    choices: Vec<String>,
}

impl GuiApp {
    pub fn new(configs: configs::Configs) -> Self {
        let choices: Vec<String> = configs.shortcuts.keys().map(|s| s.to_string()).collect();
        let mut wind = window::Window::new(100, 100, 400, 300, "shorty");
        let mut menu = menu::Choice::new(100, 100, 150, 30, None);
        for choice in choices.clone().iter() {
            menu.add_choice(choice);
        }
        wind.end();
        wind.show();
        Self {
            window: wind,
            menu,
            configs,
            choices,
        }
    }

    pub fn run(&mut self) {
        while app::wait() {
            if self.menu.value() > 0 {
                println!(
                    "Selected Option: {}",
                    &self.choices[self.menu.value() as usize - 1]
                );
            }
        }
    }
}
