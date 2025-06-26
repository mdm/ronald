use ronald_egui::{KeyEvent, KeyMapper};

#[derive(Default)]
pub struct DesktopKeyMapper {}

impl KeyMapper for DesktopKeyMapper {
    fn bind_key(
        &mut self,
        key: &str,
        input: &egui::InputState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn reset_bindings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn map_keys(&mut self, input: &egui::InputState) -> impl Iterator<Item = KeyEvent<'_>> {
        input.raw.events.iter().filter_map(|event| {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
                ..
            } = event
            {
                if *pressed {
                    log::debug!("Key pressed: {:?} with modifiers: {:?}", key, modifiers);
                    if key.name() == "Backspace" {
                        Some(KeyEvent::Pressed("Delete"))
                    } else {
                        Some(KeyEvent::Pressed(key.name()))
                    }
                } else {
                    log::debug!("Key released: {:?}", key);
                    if key.name() == "Backspace" {
                        Some(KeyEvent::Released("Delete"))
                    } else {
                        Some(KeyEvent::Released(key.name()))
                    }
                }
            } else {
                None
            }
        })
    }
}
