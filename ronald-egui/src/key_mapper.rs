use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
};

use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::frontend::KeyEvent;
use crate::utils::vectorize;

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

#[derive(Default, Serialize, Deserialize)]
pub struct KeyMap {
    #[serde(with = "vectorize")]
    host_to_guest: HashMap<HostKey, Vec<Vec<String>>>,
    guest_to_description: HashMap<String, (Option<String>, Option<String>)>,
}

pub struct KeyMapper<S>
where
    S: KeyMapStore,
{
    key_map: KeyMap,
    key_map_store: S,
    pressed_keys: HashMap<egui::Key, HostKey>,
    alt_gr_pressed: bool,
}

impl<S> KeyMapper<S>
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

        // Remove empty Vecs from host_to_guest
        self.key_map.host_to_guest.retain(|_, guest_keys| !guest_keys.is_empty());

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

impl<S> Default for KeyMapper<S>
where
    S: KeyMapStore,
{
    fn default() -> Self {
        let key_map_store = S::default();
        let key_map = key_map_store.load_key_map().unwrap_or_else(|_| {
            log::warn!("No key map found.");
            KeyMap::default()
        });

        Self {
            key_map,
            key_map_store,
            pressed_keys: HashMap::new(),
            alt_gr_pressed: false,
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
pub struct NativeKeyMapStore<'a> {
    key_map_path: &'a str,
    key_map_default_path: &'a str,
    key_map_backup_path: &'a str,
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct WebKeyMapStore;

#[cfg(not(target_arch = "wasm32"))]
impl<'a> Default for NativeKeyMapStore<'a> {
    fn default() -> Self {
        Self {
            key_map_path: "keymap.json",
            key_map_default_path: "keymap.default.json",
            key_map_backup_path: "keymap.old.json",
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> NativeKeyMapStore<'a> {
    fn try_load_from_file(&self, path: &str) -> Result<KeyMap, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(Into::into)
    }

    fn save_to_file(&self, keymap: &KeyMap, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, keymap).map_err(Into::into)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> KeyMapStore for NativeKeyMapStore<'a> {
    fn load_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>> {
        if let Ok(key_map) = self.try_load_from_file(self.key_map_path) {
            log::info!("Loaded key map from file.");
            return Ok(key_map);
        }
        match self.try_load_from_file(self.key_map_default_path) {
            Ok(key_map) => {
                log::info!("Loaded default key map from file.");
                Ok(key_map)
            }
            Err(e) => Err(e),
        }
    }

    fn save_key_map(&self, keymap: &KeyMap) -> Result<(), Box<dyn std::error::Error>> {
        let backup = match std::fs::rename(self.key_map_path, self.key_map_backup_path) {
            Ok(()) => true,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    false
                } else {
                    return Err(err.into());
                }
            }
        };

        self.save_to_file(keymap, self.key_map_path)
            .inspect(|_| {
                log::info!("Key map saved successfully.");
                if backup {
                    let _ = std::fs::remove_file(self.key_map_backup_path);
                }
            })
            .inspect_err(|err| {
                log::error!("Failed to save key map: {}", &err);
                if backup {
                    if let Ok(()) = std::fs::rename(self.key_map_backup_path, self.key_map_path) {
                        log::info!("Old key map restored successfully.");
                    };
                }
            })
    }

    fn reset_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>> {
        self.try_load_from_file(self.key_map_default_path)
    }
}

#[cfg(target_arch = "wasm32")]
impl WebKeyMapStore {
    fn load_from_server() -> Result<KeyMap, Box<dyn std::error::Error>> {
        // Use synchronous XMLHttpRequest since we need sync behavior
        let xhr = web_sys::XmlHttpRequest::new().map_err(|_| "Failed to create XMLHttpRequest")?;
        
        xhr.open_with_async("GET", "./keymap.default.json", false)
            .map_err(|_| "Failed to open request")?;
        
        xhr.send().map_err(|_| "Failed to send request")?;

        if xhr.status().map_err(|_| "Failed to get status")? != 200 {
            return Err(format!("HTTP error: {}", xhr.status().unwrap_or(0)).into());
        }

        let response_text = xhr.response_text()
            .map_err(|_| "Failed to get response text")?
            .ok_or("No response text")?;

        let keymap: KeyMap = serde_json::from_str(&response_text)?;
        Ok(keymap)
    }
}

#[cfg(target_arch = "wasm32")]
impl KeyMapStore for WebKeyMapStore {
    fn load_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>> {
        use web_sys::window;

        let window = window().ok_or("No window available")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        if let Ok(Some(json)) = storage.get_item("ronald_keymap") {
            match serde_json::from_str(&json) {
                Ok(key_map) => {
                    log::info!("Loaded key map from localStorage");
                    return Ok(key_map);
                }
                Err(e) => {
                    log::warn!(
                        "Failed to parse stored key map: {}, loading default from server",
                        e
                    );
                    // Fall through to load default from server
                }
            }
        }

        // Load default from server (either no stored keymap or parsing failed)
        match Self::load_from_server() {
            Ok(key_map) => {
                log::info!("Loaded default key map from server");
                Ok(key_map)
            }
            Err(e) => {
                log::warn!("Failed to load default key map from server: {}", e);
                log::info!("Using empty key map as fallback");
                Ok(KeyMap::default())
            }
        }
    }

    fn save_key_map(&self, keymap: &KeyMap) -> Result<(), Box<dyn std::error::Error>> {
        use web_sys::window;

        let window = window().ok_or("No window available")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        let json = serde_json::to_string(keymap)?;
        storage
            .set_item("ronald_keymap", &json)
            .map_err(|_| "Failed to save to localStorage")?;

        log::info!("Saved key map to localStorage");
        Ok(())
    }

    fn reset_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>> {
        use web_sys::window;

        let window = window().ok_or("No window available")?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        let _ = storage.remove_item("ronald_keymap");
        log::info!("Reset key map, loading default from server");

        // Load default from server
        Self::load_from_server()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockKeyMapStore;

    impl KeyMapStore for MockKeyMapStore {
        fn load_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>> {
            Ok(KeyMap::default())
        }

        fn save_key_map(&self, _keymap: &KeyMap) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn reset_key_map(&self) -> Result<KeyMap, Box<dyn std::error::Error>> {
            Ok(KeyMap::default())
        }
    }

    #[test]
    fn test_host_key_display_basic() {
        let host_key = HostKey {
            key: egui::Key::Q,
            shift: false,
            alt_gr: false,
        };
        assert_eq!(format!("{}", host_key), "Q");
    }

    #[test]
    fn test_host_key_display_with_shift() {
        let host_key = HostKey {
            key: egui::Key::Q,
            shift: true,
            alt_gr: false,
        };
        assert_eq!(format!("{}", host_key), "Shift + Q");
    }

    #[test]
    fn test_host_key_display_with_alt_gr() {
        let host_key = HostKey {
            key: egui::Key::Q,
            shift: false,
            alt_gr: true,
        };
        assert_eq!(format!("{}", host_key), "AltGr + Q");
    }

    #[test]
    fn test_host_key_display_with_both_modifiers() {
        let host_key = HostKey {
            key: egui::Key::Q,
            shift: true,
            alt_gr: true,
        };
        assert_eq!(format!("{}", host_key), "Shift + AltGr + Q");
    }

    #[test]
    fn test_record_host_key_normal_key() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let result = mapper.record_host_key(egui::Key::Q, true, egui::Modifiers::NONE);
        assert!(result.is_some());
    }

    #[test]
    fn test_record_host_key_shift_left_returns_none() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let result = mapper.record_host_key(egui::Key::ShiftLeft, true, egui::Modifiers::NONE);
        assert!(result.is_none());
    }

    #[test]
    fn test_record_host_key_shift_right_returns_none() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let result = mapper.record_host_key(egui::Key::ShiftRight, true, egui::Modifiers::NONE);
        assert!(result.is_none());
    }

    #[test]
    fn test_record_host_key_alt_right_returns_none() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let result = mapper.record_host_key(egui::Key::AltRight, true, egui::Modifiers::NONE);
        assert!(result.is_none());
    }

    #[test]
    fn test_record_host_key_alt_right_sets_alt_gr_flag() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        mapper.record_host_key(egui::Key::AltRight, true, egui::Modifiers::NONE);
        assert!(mapper.alt_gr_pressed);
    }

    #[test]
    fn test_record_host_key_with_shift_modifier() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let result = mapper.record_host_key(egui::Key::Q, true, egui::Modifiers::SHIFT);
        let host_key = result.unwrap();
        assert!(host_key.shift);
    }

    #[test]
    fn test_record_host_key_with_alt_gr_pressed() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        mapper.alt_gr_pressed = true;
        let result = mapper.record_host_key(egui::Key::Q, true, egui::Modifiers::NONE);
        let host_key = result.unwrap();
        assert!(host_key.alt_gr);
    }

    #[test]
    fn test_record_host_key_alt_right_release_clears_flag() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        mapper.alt_gr_pressed = true;
        mapper.record_host_key(egui::Key::AltRight, false, egui::Modifiers::NONE);
        assert!(!mapper.alt_gr_pressed);
    }

    #[test]
    fn test_binding_returns_none_for_unknown_key() {
        let mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        assert_eq!(mapper.binding("UNKNOWN", false), None);
    }

    #[test]
    fn test_binding_returns_normal_binding() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        mapper.key_map.guest_to_description.insert(
            "A".to_string(),
            (Some("Q".to_string()), None),
        );
        assert_eq!(mapper.binding("A", false), Some("Q"));
    }

    #[test]
    fn test_binding_returns_shifted_binding() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        mapper.key_map.guest_to_description.insert(
            "A".to_string(),
            (None, Some("Shift + Q".to_string())),
        );
        assert_eq!(mapper.binding("A", true), Some("Shift + Q"));
    }

    #[test]
    fn test_binding_returns_none_when_shifted_not_available() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        mapper.key_map.guest_to_description.insert(
            "A".to_string(),
            (Some("Q".to_string()), None),
        );
        assert_eq!(mapper.binding("A", true), None);
    }

    fn create_input_state_with_key(key: egui::Key, modifiers: egui::Modifiers, pressed: bool) -> egui::InputState {
        let events = vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        }];
        let mut raw_input = egui::RawInput::default();
        raw_input.events = events;
        let mut input_state = egui::InputState::default();
        input_state.raw = raw_input;
        input_state
    }

    #[test]
    fn test_try_set_binding_returns_false_for_no_input() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let input_state = egui::InputState::default();
        let result = mapper.try_set_binding("A", false, &input_state);
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_try_set_binding_sets_normal_binding() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        let result = mapper.try_set_binding("A", false, &input_state);
        assert_eq!(result.unwrap(), true);
        assert_eq!(mapper.binding("A", false), Some("Q"));
        
        // Verify the binding works by testing key mapping
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut received_key = None;
        mapper.map_keys(&press_input, |event| {
            if let crate::frontend::KeyEvent::Pressed(key) = event {
                received_key = Some(key.to_string());
            }
        });
        assert_eq!(received_key, Some("A".to_string()));
    }

    #[test]
    fn test_try_set_binding_sets_shifted_binding() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::SHIFT, true);
        let result = mapper.try_set_binding("A", true, &input_state);
        assert_eq!(result.unwrap(), true);
        assert_eq!(mapper.binding("A", true), Some("Shift + Q"));
        
        // Verify the binding works by testing key mapping
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::SHIFT, true);
        
        let mut received_keys = Vec::new();
        mapper.map_keys(&press_input, |event| {
            if let crate::frontend::KeyEvent::Pressed(key) = event {
                received_keys.push(key.to_string());
            }
        });
        assert_eq!(received_keys.len(), 2);
        assert!(received_keys.contains(&"A".to_string()));
        assert!(received_keys.contains(&"Shift".to_string()));
    }

    #[test]
    fn test_clear_binding_removes_normal_binding() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up a binding first using the public interface
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &input_state).unwrap();
        
        // Verify binding exists
        assert_eq!(mapper.binding("A", false), Some("Q"));
        
        // Clear the binding
        let result = mapper.clear_binding("A", false);
        assert!(result.is_ok());
        assert_eq!(mapper.binding("A", false), None);
        
        // Verify the binding no longer works by testing key mapping
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut received_key = None;
        mapper.map_keys(&press_input, |event| {
            if let crate::frontend::KeyEvent::Pressed(key) = event {
                received_key = Some(key.to_string());
            }
        });
        assert_eq!(received_key, None);
    }

    #[test]
    fn test_clear_binding_removes_shifted_binding() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up a shifted binding first using the public interface
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::SHIFT, true);
        mapper.try_set_binding("A", true, &input_state).unwrap();
        
        // Verify binding exists
        assert_eq!(mapper.binding("A", true), Some("Shift + Q"));
        
        // Clear the shifted binding
        let result = mapper.clear_binding("A", true);
        assert!(result.is_ok());
        assert_eq!(mapper.binding("A", true), None);
        
        // Verify the binding no longer works by testing key mapping
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::SHIFT, true);
        
        let mut event_received = false;
        mapper.map_keys(&press_input, |_| {
            event_received = true;
        });
        // Should receive no events since the entire binding was removed
        assert!(!event_received);
    }

    #[test]
    fn test_reset_all_bindings_clears_existing_bindings() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up a binding first using the public interface
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &input_state).unwrap();
        
        // Verify binding exists
        assert_eq!(mapper.binding("A", false), Some("Q"));
        
        // Reset all bindings
        let result = mapper.reset_all_bindings();
        assert!(result.is_ok());
        assert_eq!(mapper.binding("A", false), None);
        
        // Verify the binding no longer works by testing key mapping
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut event_received = false;
        mapper.map_keys(&press_input, |_| {
            event_received = true;
        });
        assert!(!event_received);
    }

    #[test]
    fn test_map_keys_calls_callback_on_key_press() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up a binding first using the public interface
        let setup_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &setup_input).unwrap();
        
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut received_key = None;
        mapper.map_keys(&input_state, |event| {
            match event {
                crate::frontend::KeyEvent::Pressed(key) => received_key = Some(("Pressed", key.to_string())),
                crate::frontend::KeyEvent::Released(key) => received_key = Some(("Released", key.to_string())),
            }
        });
        assert_eq!(received_key, Some(("Pressed", "A".to_string())));
    }

    #[test]
    fn test_map_keys_calls_callback_on_key_release() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up a binding first using the public interface
        let setup_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &setup_input).unwrap();
        
        // First press the key to track it
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.map_keys(&press_input, |_| {}); // Just to track the key as pressed
        
        // Now release the key
        let release_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, false);
        
        let mut received_key = None;
        mapper.map_keys(&release_input, |event| {
            match event {
                crate::frontend::KeyEvent::Pressed(key) => received_key = Some(("Pressed", key.to_string())),
                crate::frontend::KeyEvent::Released(key) => received_key = Some(("Released", key.to_string())),
            }
        });
        assert_eq!(received_key, Some(("Released", "A".to_string())));
    }

    #[test]
    fn test_multiple_guest_keys_bound_to_same_host_key() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up binding for A to Q
        let setup_input_a = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &setup_input_a).unwrap();
        
        // Set up binding for B to the same key Q (this should add to the existing binding)
        let setup_input_b = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("B", false, &setup_input_b).unwrap();
        
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut received_keys = Vec::new();
        mapper.map_keys(&input_state, |event| {
            match event {
                crate::frontend::KeyEvent::Pressed(key) => received_keys.push(("Pressed", key.to_string())),
                crate::frontend::KeyEvent::Released(key) => received_keys.push(("Released", key.to_string())),
            }
        });
        
        assert_eq!(received_keys.len(), 2);
        assert!(received_keys.contains(&("Pressed", "A".to_string())));
        assert!(received_keys.contains(&("Pressed", "B".to_string())));
    }

    #[test]
    fn test_rebinding_guest_key_leaves_other_bindings_intact() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up initial bindings: Q maps to both A and B
        let setup_input_a = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &setup_input_a).unwrap();
        
        let setup_input_b = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("B", false, &setup_input_b).unwrap();

        // Rebind A to W (leaving B still bound to Q)
        let input_state = create_input_state_with_key(egui::Key::W, egui::Modifiers::NONE, true);
        let result = mapper.try_set_binding("A", false, &input_state);
        assert_eq!(result.unwrap(), true);

        // Verify A is now bound to W
        assert_eq!(mapper.binding("A", false), Some("W"));
        
        // Verify B is still bound to Q
        assert_eq!(mapper.binding("B", false), Some("Q"));

        // Test that pressing W triggers A and pressing Q triggers B
        let press_w_input = create_input_state_with_key(egui::Key::W, egui::Modifiers::NONE, true);
        
        let mut received_key = None;
        mapper.map_keys(&press_w_input, |event| {
            if let crate::frontend::KeyEvent::Pressed(key) = event {
                received_key = Some(key.to_string());
            }
        });
        assert_eq!(received_key, Some("A".to_string()));
        
        // Test that pressing Q still triggers B
        let press_q_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut received_key = None;
        mapper.map_keys(&press_q_input, |event| {
            if let crate::frontend::KeyEvent::Pressed(key) = event {
                received_key = Some(key.to_string());
            }
        });
        assert_eq!(received_key, Some("B".to_string()));
    }

    #[test]
    fn test_clear_binding_leaves_other_bindings_for_same_host_key_intact() {
        let mut mapper: KeyMapper<MockKeyMapStore> = KeyMapper::default();
        
        // Set up initial bindings: Q maps to both A and B
        let setup_input_a = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("A", false, &setup_input_a).unwrap();
        
        let setup_input_b = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        mapper.try_set_binding("B", false, &setup_input_b).unwrap();

        // Clear binding for A (should leave B intact)
        let result = mapper.clear_binding("A", false);
        assert!(result.is_ok());

        // Verify A binding is cleared
        assert_eq!(mapper.binding("A", false), None);

        // Verify B binding remains intact
        assert_eq!(mapper.binding("B", false), Some("Q"));
        
        // Test that pressing Q still triggers B but not A
        let press_input = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        
        let mut received_key = None;
        mapper.map_keys(&press_input, |event| {
            if let crate::frontend::KeyEvent::Pressed(key) = event {
                received_key = Some(key.to_string());
            }
        });
        assert_eq!(received_key, Some("B".to_string()));
    }
}
