use std::collections::HashMap;

use ronald_egui::{KeyEvent, KeyMapper};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HostKey {
    key: egui::Key,
    modifiers: egui::Modifiers,
}

#[derive(Default)]
pub struct DesktopKeyMapper {
    keymap: HashMap<HostKey, Vec<String>>,
}

impl KeyMapper for DesktopKeyMapper {
    fn bind_key(
        &mut self,
        guest_key: &str,
        input: &egui::InputState,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        for event in &input.raw.events {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
                ..
            } = event
            {
                // TODO: handle AltGr modifier
                let host_key = HostKey {
                    key: *key,
                    modifiers: *modifiers,
                };
                if *pressed {
                    self.keymap
                        .entry(host_key)
                        .or_default()
                        .push(guest_key.to_string());
                }
            }
        }

        Ok(false)
    }

    fn reset_bindings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn map_keys(&mut self, input: &egui::InputState) -> impl Iterator<Item = KeyEvent<'_>> {
        input.raw.events.iter().flat_map(|event| {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
                ..
            } = event
            {
                let host_key = HostKey {
                    key: *key,
                    modifiers: *modifiers,
                };
                if *pressed {
                    if key.name() == "Backspace" {
                        vec![KeyEvent::Pressed("Delete")]
                    } else {
                        // let guest_keys = self.keymap.get(&host_key).unwrap_or(&vec![]);
                        // log::debug!("Mapping host key {:?} to guest key {:?}", key, guest_keys);
                        // guest_keys
                        //     .iter()
                        //     .map(|guest_key| KeyEvent::Pressed(guest_key.as_str()))
                        //     .collect()
                        vec![KeyEvent::Pressed(key.name())]
                    }
                } else {
                    log::debug!("Key released: {:?} ({:?})", key, modifiers);
                    if key.name() == "Backspace" {
                        vec![KeyEvent::Released("Delete")]
                    } else {
                        vec![KeyEvent::Released(key.name())]
                    }
                }
            } else {
                vec![]
            }
        })
    }
}
