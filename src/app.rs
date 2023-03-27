use iced::{Application, button, Column, Command, Element, Settings, Text};
use iced::widget::dropdown_list::{self, DropdownList};
use crate::configs;

struct Shorty {
    shortcuts: Vec<String>,
    selected: usize,
    dropdown_state: dropdown_list::State,
    btn_state: button::State,
    configs: configs::Configs,
}

#[derive(Debug, Clone)]
enum Msg {
    NewSelection(usize),
    BtnPressed,
}

impl Application for Shorty {
    type Executor = iced::executor::Tokio;
    type Message = Msg;

    fn new() -> (Shorty, Command<Self::Message>) {
        let configs = configs::Configs::new();
        let shortcuts = configs.shortcuts.keys().collect();
        (
            Shorty {
                shortcuts,
                selected: 0,
                dropdown_state: dropdown_list::State::default(),
                btn_state: button::State::new(),
                configs,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("shorty")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Msg::NewSelection(idx) => {
                self.selected = idx;
            }
            Msg::BtnPressed => {
                todo!()
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let dropdown = DropdownList::new(
            &mut self.dropdown_state,
            &self.shortcuts,
            Some(self.selected),
            Msg::NewSelection,
        );

        let button = button::Button::new(&mut self.btn_state, Text::new("go"))
            .on_press(Msg::BtnPressed);

        Column::new().spacing(20).push(dropdown).push(button).into()
    }
}
