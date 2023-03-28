use shorty::configs;
use shorty::gui;
use anyhow::Result;
use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};

fn spawn_app() -> Result<()> {
    let configs = configs::Configs::new();
    gui::GuiApp::new(configs).spawn()?;
    Ok(())
}

fn main() {
    let state = Arc::new(Mutex::new((false, false, false))); // (Ctrl, Alt, P)

    let state_clone = Arc::clone(&state);
    let _handler = std::thread::spawn(move || {
        if let Err(error) = listen(move |event| callback(event.event_type, &state_clone)) {
            println!("Error: {:?}", error);
        }
    });

    loop {
        let (ctrl, alt, p) = { *state.lock().unwrap() };
        if ctrl && alt && p {
            spawn_app().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn callback(event: EventType, state: &Arc<Mutex<(bool, bool, bool)>>) {
    match event {
        EventType::KeyPress(key) | EventType::KeyRelease(key) => {
            let mut state = state.lock().unwrap();
            match key {
                Key::ControlLeft | Key::ControlRight => state.0 = event.is_key_press(),
                Key::Alt => state.1 = event.is_key_press(),
                Key::KeyV => state.2 = event.is_key_press(),
                _ => (),
            }
        }
        _ => (),
    }
}

trait KeyEvent {
    fn is_key_press(&self) -> bool;
}

impl KeyEvent for EventType {
    fn is_key_press(&self) -> bool {
        matches!(*self, EventType::KeyPress(_))
    }
}
