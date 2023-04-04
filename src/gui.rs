use crate::configs;
use anyhow::Result;
use fltk::{prelude::*, *, enums::*, window};
use std::{rc::Rc, cell::RefCell};

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
        // Create a new window for the input field and drop-down menu
        let mut input_wind = window::Window::new(x_offset, y_offset, self.configs.settings.width + MENU_OFFSET, 30, "shrtcut");

        // Create the input field
        let mut input_buffer = input::Input::new(MENU_OFFSET, MENU_OFFSET, menu_width, 25, None);
        input_buffer.set_trigger(CallbackTrigger::Changed);

        // Create the drop-down menu
        let menu_rc = Rc::new(RefCell::new(menu::Choice::new(MENU_OFFSET, MENU_OFFSET + 25, menu_width, menu_height, None)));
        for choice in self.choices.clone().iter() {
            menu_rc.borrow_mut().add_choice(choice);
        }

        let menu_rc_clone = menu_rc.clone();
        let choices_clone = self.choices.clone();
        input_buffer.set_callback(move |input_buffer| {
            let input_text = input_buffer.value();
            let filtered_choices = choices_clone.iter().filter(|c| c.starts_with(&input_text)).map(|c| c.to_string()).collect::<Vec<String>>();

            menu_rc_clone.borrow_mut().clear();
            for choice in filtered_choices {
                menu_rc_clone.borrow_mut().add_choice(&choice);
            }
        });

        input_wind.end();
        input_wind.show();
        while app::wait() {
            if menu_rc.borrow().value() >= 0 {
                self.configs.use_shortcut(self.choices[menu_rc.borrow().value() as usize].clone())?;
                menu_rc.borrow_mut().hide();
                input_wind.hide();
                break;
            }
        }
        app::quit();
        Ok(())
    }
}
