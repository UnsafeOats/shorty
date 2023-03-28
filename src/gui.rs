use crate::configs;
use anyhow::Result;
use fltk::{prelude::*, *};

pub struct GuiApp {
    configs: configs::Configs,
    choices: Vec<String>,
}

impl GuiApp {
    pub fn new(configs: configs::Configs) -> Self {
        let choices: Vec<String> = configs.shortcuts.keys().map(|s| s.to_string()).collect();
        Self {
            configs,
            choices,
        }
    }

    pub fn spawn(&self) -> Result<()> {
        let mut wind = window::Window::new(450, 450, 400, 60, "shorty");
        let mut menu = menu::Choice::new(20, 20, 350, 30, None);
        for choice in self.choices.clone().iter() {
            menu.add_choice(choice);
        }
        wind.end();
        wind.show();
        while app::wait() {
            if menu.value() >= 0 {
                self.configs.copy_to_clipboard(self.choices[menu.value() as usize].clone())?;
                println!("worked {:?}", self.choices[menu.value() as usize]);
                menu.hide();
                wind.hide();
                app::quit();
                return Ok(());
            }
        }
        Ok(())
    }
}
