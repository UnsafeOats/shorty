use shorty::configs;
use shorty::gui;
use anyhow::Result;

fn spawn_app() -> Result<()> {
    let configs = configs::Configs::new();
    gui::GuiApp::new(configs).spawn()?;
    Ok(())
}

fn main() {
    match spawn_app() {
        Ok(_) => (),
        Err(e) => println!("[error] {:?}", e),
    }
}
