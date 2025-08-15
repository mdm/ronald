use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use ronald_egui::{KeyMap, KeyMapStore};

pub struct DesktopKeyMapStore<'a> {
    key_map_path: &'a str,
    key_map_default_path: &'a str,
    key_map_backup_path: &'a str,
}

impl<'a> Default for DesktopKeyMapStore<'a> {
    fn default() -> Self {
        Self {
            key_map_path: "keymap.json",
            key_map_default_path: "keymap.default.json",
            key_map_backup_path: "keymap.old.json",
        }
    }
}

impl<'a> DesktopKeyMapStore<'a> {
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

impl<'a> KeyMapStore for DesktopKeyMapStore<'a> {
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