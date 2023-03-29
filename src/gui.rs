use crate::configs;
use anyhow::Result;
use fltk::{prelude::*, *};

pub struct GuiApp {
    configs: configs::Configs,
    choices: Vec<String>,
}

impl GuiApp {
    pub fn new(configs: configs::Configs) -> Self {
        let first_choice = match configs.settings.default.clone() {
            Some(s) => if configs.shortcuts.contains_key(&s) {
                s
            } else {
                "".to_string()
            },
            None => "".to_string(),
        };
        let mut choices = vec![first_choice];
        let mut additional_choices: Vec<String> = configs.shortcuts.keys().filter(|s| s != &&choices[0]).map(|s| s.to_string()).collect();
        additional_choices.sort();
        choices.append(&mut additional_choices);
        Self {
            configs: configs,
            choices,
        }
    }

    pub fn spawn(&self) -> Result<()> {
        let (x, y) = app::get_mouse();
        const MENU_OFFSET: i32 = 5;
        let x_offset = x - self.configs.settings.width + 5;
        let y_offset = y - self.configs.settings.height + 5;
        let menu_width = self.configs.settings.width - MENU_OFFSET;
        let menu_height = self.configs.settings.height - MENU_OFFSET;
        let mut wind = window::Window::new(x_offset, y_offset, self.configs.settings.width + MENU_OFFSET, self.configs.settings.height + MENU_OFFSET, "shrtcut");
        let mut menu = menu::Choice::new(MENU_OFFSET, MENU_OFFSET, menu_width, menu_height, None);
        for choice in self.choices.clone().iter() {
            menu.add_choice(choice);
        }
        wind.end();
        wind.show();
        while app::wait() {
            if menu.value() >= 0 {
                self.configs.copy_to_clipboard(self.choices[menu.value() as usize].clone())?;
                menu.hide();
                wind.hide();
                break;
            }
        }
        app::quit();
        Ok(())
    }
}
