use crate::configs;
use crate::gui;

fn main() {
    let configs = configs::Configs::new();
    let app = gui::GuiApp::new(configs);
}
