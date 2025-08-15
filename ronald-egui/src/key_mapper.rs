use std::{
    collections::HashMap,
    fmt::Display,
};

use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::frontend::KeyEvent;

pub trait KeyMapStore: Default {
    fn load_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>>;
    fn save_key_map(&self, keymap: &KeyMap) -> Result<(), Box<dyn std::error::Error>>;
    fn reset_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct HostKey {
    key: egui::Key,
    shift: bool,
    alt_gr: bool,
}

impl Display for HostKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.shift {
            write!(f, "Shift + ")?;
        }

        if self.alt_gr {
            write!(f, "AltGr + ")?;
        }

        write!(f, "{}", self.key.name())
    }
}

struct KeyMapConfig<'c> {
    key_map_path: &'c str,
    key_map_default_path: &'c str,
    key_map_backup_path: &'c str,
}

impl<'c> Default for KeyMapConfig<'c> {
    fn default() -> Self {
        Self {
            key_map_path: "keymap.json",
            key_map_default_path: "keymap.default.json",
            key_map_backup_path: "keymap.old.json",
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct KeyMap {
    #[serde(with = "vectorize")]
    host_to_guest: HashMap<HostKey, Vec<Vec<String>>>,
    guest_to_description: HashMap<String, (Option<String>, Option<String>)>,
}


pub struct KeyMapper<'c, S> 
where
    S: KeyMapStore,
{
    config: KeyMapConfig<'c>,
    key_map: KeyMap,
    key_map_store: S,
    pressed_keys: HashMap<egui::Key, HostKey>,
    alt_gr_pressed: bool,
}

impl<'c, S> KeyMapper<'c, S>
where
    S: KeyMapStore,
{

    /// We only treat Shift and AltGr as modifiers as these are commonly used to compose alternate
    /// characters on the host system. All other modifiers are treated as key presses and should be
    /// handled by the guest system.
    fn record_host_key(
        &mut self,
        physical_key: egui::Key,
        pressed: bool,
        modifiers: egui::Modifiers,
    ) -> Option<HostKey> {
        match physical_key {
            egui::Key::ShiftLeft | egui::Key::ShiftRight => None,
            egui::Key::AltRight => {
                self.alt_gr_pressed = pressed;
                None
            }
            key => Some(HostKey {
                key,
                shift: modifiers.contains(egui::Modifiers::SHIFT),
                alt_gr: self.alt_gr_pressed,
            }),
        }
    }

    pub fn binding(&self, guest_key: &str, shifted: bool) -> Option<&str> {
        self.key_map
            .guest_to_description
            .get(guest_key)
            .and_then(|bindings| {
                if shifted {
                    bindings.1.as_deref()
                } else {
                    bindings.0.as_deref()
                }
            })
    }

    pub fn try_set_binding(
        &mut self,
        guest_key: &str,
        shifted: bool,
        input: &egui::InputState,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        for event in &input.raw.events {
            if let egui::Event::Key {
                physical_key: Some(physical_key),
                pressed,
                modifiers,
                ..
            } = event
                && let Some(host_key) = self.record_host_key(*physical_key, *pressed, *modifiers)
                && *pressed
            {
                // Clear any existing binding for the guest key.
                self.clear_binding(guest_key, shifted)?;

                if shifted {
                    self.key_map
                        .host_to_guest
                        .entry(host_key)
                        .or_default()
                        .push(vec![guest_key.to_string(), "Shift".to_string()]);

                    self.key_map
                        .guest_to_description
                        .entry(guest_key.to_string())
                        .or_default()
                        .1 = Some(host_key.to_string());
                } else {
                    self.key_map
                        .host_to_guest
                        .entry(host_key)
                        .or_default()
                        .push(vec![guest_key.to_string()]);

                    self.key_map
                        .guest_to_description
                        .entry(guest_key.to_string())
                        .or_default()
                        .0 = Some(host_key.to_string());
                }

                self.key_map_store.save_key_map(&self.key_map)?;

                return Ok(true);
            }
        }

        // TODO: handle modifier-only input
        // TODO: handle gamepad input

        Ok(false)
    }

    pub fn clear_binding(
        &mut self,
        guest_key: &str,
        shifted: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (_, guest_keys) in self.key_map.host_to_guest.iter_mut() {
            guest_keys.retain(|old_binding| {
                !(old_binding.iter().any(|old_key| old_key == guest_key)
                    && (!shifted || old_binding.iter().any(|old_key| old_key == "Shift")))
            });
        }

        if shifted {
            self.key_map
                .guest_to_description
                .entry(guest_key.to_string())
                .and_modify(|bindings| {
                    bindings.1 = None;
                });
        } else {
            self.key_map
                .guest_to_description
                .entry(guest_key.to_string())
                .and_modify(|bindings| {
                    bindings.0 = None;
                });
        }

        self.key_map_store.save_key_map(&self.key_map)?;

        Ok(())
    }

    pub fn reset_all_bindings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let key_map = self.key_map_store.reset_key_map()?;

        self.key_map = key_map;
        self.key_map_store.save_key_map(&self.key_map)?;

        Ok(())
    }

    pub fn map_keys(&mut self, input: &egui::InputState, mut callback: impl FnMut(KeyEvent)) {
        for host_event in &input.raw.events {
            if let egui::Event::Key {
                physical_key: Some(physical_key),
                pressed,
                modifiers,
                ..
            } = host_event
                && let Some(host_key) = self.record_host_key(*physical_key, *pressed, *modifiers)
            {
                if *pressed {
                    if let Some(guest_keys) = self.key_map.host_to_guest.get(&host_key) {
                        log::debug!(
                            "Mapping host key {physical_key:?} to guest keys {guest_keys:?}"
                        );
                        self.pressed_keys.insert(*physical_key, host_key);
                        for guest_key in guest_keys {
                            for guest_event in guest_key {
                                callback(KeyEvent::Pressed(guest_event));
                            }
                        }
                    }
                } else {
                    log::debug!("Host key released: {physical_key:?}");
                    if let Some(host_key) = self.pressed_keys.remove(physical_key)
                        && let Some(guest_keys) = self.key_map.host_to_guest.get(&host_key)
                    {
                        for guest_key in guest_keys {
                            for guest_event in guest_key {
                                callback(KeyEvent::Released(guest_event))
                            }
                        }
                    }
                }
            }
        }

        // TODO: handle gamepad input
    }
}

impl<'c, S> Default for KeyMapper<'c, S>
where
    S: KeyMapStore,
{
    fn default() -> Self {
        let config = KeyMapConfig::default();
        let key_map_store = S::default();
        let key_map = key_map_store.load_key_map().unwrap_or_else(|_| {
            log::warn!("No key map found.");
            KeyMap::default()
        });

        Self {
            config,
            key_map,
            key_map_store,
            pressed_keys: HashMap::new(),
            alt_gr_pressed: false,
        }
    }
}

mod vectorize {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::iter::FromIterator;

    pub fn serialize<'a, T, K, V, S>(target: T, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: IntoIterator<Item = (&'a K, &'a V)>,
        K: Serialize + 'a,
        V: Serialize + 'a,
    {
        let container: Vec<_> = target.into_iter().collect();
        serde::Serialize::serialize(&container, ser)
    }

    pub fn deserialize<'de, T, K, V, D>(des: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromIterator<(K, V)>,
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        let container: Vec<_> = serde::Deserialize::deserialize(des)?;
        Ok(T::from_iter(container))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

