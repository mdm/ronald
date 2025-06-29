use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{Deserialize, Serialize};

use ronald_egui::{KeyEvent, KeyMapper};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct HostKey {
    key: egui::Key,
    modifiers: egui::Modifiers,
}

impl Display for HostKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifiers.contains(egui::Modifiers::SHIFT) {
            write!(f, "Shift + ")?;
        }

        if self.modifiers.contains(egui::Modifiers::CTRL) {
            write!(f, "Ctrl + ")?;
        }

        if self.modifiers.contains(egui::Modifiers::MAC_CMD) {
            write!(f, "Command + ")?;
        }

        if self.modifiers.contains(egui::Modifiers::ALT) {
            write!(f, "Alt + ")?;
        }

        write!(f, "{}", self.key.name())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DesktopKeyMapper {
    key_map: HashMap<HostKey, Vec<String>>,
    reverse_key_map: HashMap<String, (Option<String>, Option<String>)>,
    #[serde(skip)]
    pressed_keys: HashMap<egui::Key, HostKey>,
}

impl DesktopKeyMapper {
    fn try_from_file<P>(path: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let backup = match std::fs::rename("keymap.json", "keymap.old.json") {
            Ok(()) => true,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    false
                } else {
                    Err(err)?
                }
            }
        };

        self.write_key_map()
            .inspect(|_| {
                log::info!("Key map saved successfully.");
                if backup {
                    let _ = std::fs::remove_file("keymap.old.json");
                }
            })
            .inspect_err(|err| {
                log::error!("Failed to save key map: {}", &err);
                if !backup {
                    return;
                }
                if let Ok(()) = std::fs::rename("keymap.old.json", "keymap.json") {
                    log::info!("Old key map restored successfully.");
                };
            })
    }

    fn write_key_map(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create("keymap.json")?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &self)?;

        Ok(())
    }
}

impl Default for DesktopKeyMapper {
    fn default() -> Self {
        if let Ok(loaded) = Self::try_from_file("keymap.json") {
            log::info!("Loaded key map from file.");
            return loaded;
        }
        if let Ok(loaded) = Self::try_from_file("keymap.default.json") {
            log::info!("Loaded default key map from file.");
            return loaded;
        }

        log::warn!("No key map found.");
        Self {
            key_map: HashMap::new(),
            reverse_key_map: HashMap::new(),
            pressed_keys: HashMap::new(),
        }
    }
}

impl KeyMapper for DesktopKeyMapper {
    fn binding(&self, guest_key: &str, shifted: bool) -> Option<&str> {
        self.reverse_key_map.get(guest_key).and_then(|bindings| {
            if shifted {
                bindings.1.as_deref()
            } else {
                bindings.0.as_deref()
            }
        })
    }

    fn try_set_binding(
        &mut self,
        guest_key: &str,
        shifted: bool,
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
                    // We don't simply insert the key binding here, because we want to allow
                    // binding multiple guest keys to the same host key, e.g. map the host's
                    // cursor keys both to the guest's cursor keys and the joystick.
                    if shifted {
                        self.key_map
                            .entry(host_key)
                            .or_default()
                            .retain(|old_binding| {
                                ![guest_key, "Shift"].contains(&old_binding.as_str())
                            });

                        self.key_map
                            .entry(host_key)
                            .or_default()
                            .extend_from_slice(&[guest_key.to_string(), "Shift".to_string()]);

                        self.reverse_key_map
                            .entry(guest_key.to_string())
                            .or_default()
                            .1 = Some(host_key.to_string());
                    } else {
                        self.key_map
                            .entry(host_key)
                            .or_default()
                            .retain(|old_binding| guest_key != old_binding.as_str());

                        self.key_map
                            .entry(host_key)
                            .or_default()
                            .push(guest_key.to_string());

                        self.reverse_key_map
                            .entry(guest_key.to_string())
                            .or_default()
                            .0 = Some(host_key.to_string());
                    }

                    self.save()?;

                    return Ok(true);
                }
            }
        }

        // TODO: handle modifier-only input
        // TODO: handle gamepad input

        Ok(false)
    }

    fn reset_bindings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let loaded = Self::try_from_file("keymap.default.json")?;

        self.key_map = loaded.key_map;
        self.reverse_key_map = loaded.reverse_key_map;
        self.save()?;

        Ok(())
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
                        log::debug!("Mapping host key {:?} to guest keys {:?}", key, guest_keys);
                        self.pressed_keys.insert(*key, host_key);
                        for guest_key in guest_keys {
                            callback(KeyEvent::Pressed(guest_key));
                        }
                    }
                    log::debug!("Key released: {:?} ({:?})", key, modifiers);
                    if let Some(host_key) = self.pressed_keys.get(key) {
                        if let Some(keys) = self.key_map.get(host_key) {
                            for key in keys {
                                callback(KeyEvent::Released(key))
                            }
                        }
                    }
                }
            }
        }

        // TODO: handle modifier-only input
        // TODO: handle gamepad input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
