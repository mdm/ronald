use std::collections::HashMap;

use ronald_egui::{KeyEvent, KeyMapper};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HostKey {
    key: egui::Key,
    modifiers: egui::Modifiers,
}

#[derive(Default)]
pub struct DesktopKeyMapper {
    key_map: HashMap<HostKey, Vec<String>>,
    pressed_keys: HashMap<egui::Key, HostKey>,
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
                    self.key_map
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

    fn map_keys(&mut self, input: &egui::InputState, mut callback: impl FnMut(KeyEvent)) {
        for event in &input.raw.events {
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
                    if let Some(guest_keys) = self.key_map.get(&host_key) {
                        log::debug!("Mapping host key {:?} to guest key {:?}", key, guest_keys);
                        self.pressed_keys.insert(*key, host_key);
                        for guest_key in guest_keys {
                            callback(KeyEvent::Pressed(guest_key));
                        }
                    }
                    // if key.name() == "Backspace" {
                    //     callback(KeyEvent::Pressed("Delete"))
                    // } else {
                    //     callback(KeyEvent::Pressed(key.name()))
                    // }
                } else {
                    log::debug!("Key released: {:?} ({:?})", key, modifiers);
                    if let Some(host_key) = self.pressed_keys.get(key) {
                        if let Some(keys) = self.key_map.get(host_key) {
                            for key in keys {
                                callback(KeyEvent::Released(key))
                            }
                        }
                    }
                    // if key.name() == "Backspace" {
                    //     callback(KeyEvent::Released("Delete"))
                    // } else {
                    //     callback(KeyEvent::Released(key.name()))
                    // }
                }
            }
        }
    }
}
