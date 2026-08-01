use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::LazyLock;

use eframe::egui;

use ronald_core::system::memory::RomSlot;
pub use ronald_core::system::{CpcModel, CrtcType, DiskDrives, SystemConfig as CoreSystemConfig};
use serde::{Deserialize, Serialize};

use crate::colors;
use crate::utils::{
    files::pick_folder,
    sync::{Shared, SharedExt, shared},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SystemConfig {
    model: CpcModel,
    crtc: CrtcType,
    disk_drives: DiskDrives,
    rom_folder: Option<PathBuf>,
    preferred_language: RomLanguage,
    auto_config: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for SystemConfig {
    fn default() -> Self {
        let rom_folder = directories::ProjectDirs::from("dev", "int82", "ronald")
            .map(|dirs| dirs.data_dir().join("roms"));

        // TODO: change defaults
        Self {
            model: CpcModel::Cpc464,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::One,
            rom_folder,
            preferred_language: RomLanguage::English,
            auto_config: true,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for SystemConfig {
    fn default() -> Self {
        let rom_folder = None;

        // TODO: change defaults
        Self {
            model: CpcModel::Cpc464,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::One,
            rom_folder,
            preferred_language: RomLanguage::English,
            auto_config: true,
        }
    }
}

impl TryFrom<SystemConfig> for CoreSystemConfig {
    type Error = anyhow::Error;

    fn try_from(value: SystemConfig) -> Result<Self, Self::Error> {
        let roms = HashMap::new();

        Ok(Self {
            model: value.model,
            crtc: value.crtc,
            disk_drives: value.disk_drives,
            roms,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
enum RomLanguage {
    Danish,
    #[default]
    English,
    French,
    Spanish,
}

impl Display for RomLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Danish => write!(f, "Danish"),
            Self::English => write!(f, "English"),
            Self::French => write!(f, "French"),
            Self::Spanish => write!(f, "Spanish"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
enum RomVariant {
    Language(RomLanguage),
    Other(String),
}

impl Display for RomVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Language(lang) => write!(f, "{}", lang),
            Self::Other(desc) => write!(f, "{}", desc),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct RomKey(PathBuf);

#[cfg(target_arch = "wasm32")]
struct RomKey(Vec<u8>);

struct RomConfig {
    key: RomKey,
    slot: RomSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct RomInfo {
    name: String,
    hash: Vec<u8>,
    variant: Option<RomVariant>,
    slot: Option<RomSlot>,
}

impl From<(&str, &str, Option<RomVariant>, Option<RomSlot>)> for RomInfo {
    fn from(tuple: (&str, &str, Option<RomVariant>, Option<RomSlot>)) -> Self {
        let (name, hash_hex, variant, slot) = tuple;
        // Panic is okay here because this is a hardcoded value and should never fail to decode.
        let hash = hex::decode(hash_hex)
            .unwrap_or_else(|_| panic!("Failed to decode ROM hash for {}: {}", name, hash_hex));
        Self {
            name: name.to_string(),
            variant,
            hash,
            slot,
        }
    }
}

struct AutoConfigRule {
    models: &'static [CpcModel],
    requires_disk_drive: bool,
}

struct OriginalRom {
    info: RomInfo,
    auto_config_rule: AutoConfigRule,
}

static ORIGINAL_ROMS: LazyLock<[OriginalRom; 6]> = LazyLock::new(|| {
    [
        OriginalRom {
            info: RomInfo::from((
                "CPC 464 OS",
                "b57872c97d569a1968816fcaf359de4b6bf5c188e7dbd0954761cc552b25d327",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 664 OS",
                "b57872c97d569a1968816fcaf359de4b6bf5c188e7dbd0954761cc552b25d327",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc664],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 6128 OS",
                "b57872c97d569a1968816fcaf359de4b6bf5c188e7dbd0954761cc552b25d327",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.0",
                "ff6fbb6e12808e7d32c9217813d730df7321b4b5e499a45e3eac21b421dcf729",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.1",
                "ff6fbb6e12808e7d32c9217813d730df7321b4b5e499a45e3eac21b421dcf729",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc664, CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "AMSDOS 0.5",
                "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7",
                None,
                Some(RomSlot::Upper(7)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464, CpcModel::Cpc664, CpcModel::Cpc6128],
                requires_disk_drive: true,
            },
        },
    ]
});

#[derive(Debug, Default, Clone, Copy)]
enum Tab {
    #[default]
    Hardware,
    Rom,
}

#[derive(Debug)]
pub struct SystemConfigModal {
    pub show: bool,
    tab: Tab,
    changed_config: Option<SystemConfig>,
    picked_rom_folder: Shared<Option<PathBuf>>,
}

impl Default for SystemConfigModal {
    fn default() -> Self {
        Self {
            show: false,
            tab: Tab::Hardware,
            changed_config: None,
            picked_rom_folder: shared(None),
        }
    }
}

impl SystemConfigModal {
    pub fn ui(&mut self, ui: &mut egui::Ui, config: &mut SystemConfig) -> bool {
        if !self.show {
            return false;
        }

        let mut config_changed = false;

        // Initialize temp config if not already set
        if self.changed_config.is_none() {
            self.changed_config = Some(config.clone());
        }

        self.handle_picked_rom_folder();

        egui::Modal::new("system_config_modal".into()).show(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(10.0);
                ui.heading("System Configuration");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(matches!(self.tab, Tab::Hardware), "Hardware")
                        .clicked()
                    {
                        self.tab = Tab::Hardware;
                    }
                    if ui
                        .selectable_label(matches!(self.tab, Tab::Rom), "System ROMs")
                        .clicked()
                    {
                        self.tab = Tab::Rom;
                    }
                });
                ui.separator();

                match self.tab {
                    Tab::Hardware => self.render_hardware_config(ui),
                    Tab::Rom => self.render_rom_config(ui),
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        if let Some(changed) = self.changed_config.take() {
                            config_changed = *config != changed;
                            *config = changed;
                        }
                        self.show = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.show = false;
                        self.changed_config = None;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Restore Defaults").clicked() {
                            self.changed_config = Some(SystemConfig::default());
                        }
                    });
                });

                ui.add_space(10.0);
            });
        });

        config_changed
    }

    fn access_config(&self) -> &SystemConfig {
        self.changed_config
            .as_ref()
            .expect("changed_config should be initialized")
    }

    fn access_config_mut(&mut self) -> &mut SystemConfig {
        self.changed_config
            .as_mut()
            .expect("changed_config should be initialized")
    }

    fn render_hardware_config(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("Model:");
                });
                ui.group(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        if let Some(config) = &mut self.changed_config {
                            ui.radio_value(&mut config.model, CpcModel::Cpc464, "Amstrad CPC 464");
                            ui.radio_value(&mut config.model, CpcModel::Cpc664, "Amstrad CPC 664");
                            ui.radio_value(
                                &mut config.model,
                                CpcModel::Cpc6128,
                                "Amstrad CPC 6128",
                            );
                        }
                    });
                });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label("Enabled Disk Drives:");
                });
                ui.group(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        if let Some(config) = &mut self.changed_config {
                            let none_enabled = config.model == CpcModel::Cpc464;

                            // Ensure we don't have None selected for models that require at least one drive
                            if !none_enabled && config.disk_drives == DiskDrives::None {
                                config.disk_drives = DiskDrives::One;
                            }

                            ui.add_enabled_ui(none_enabled, |ui| {
                                ui.radio_value(
                                    &mut config.disk_drives,
                                    DiskDrives::None,
                                    DiskDrives::None.to_string(),
                                );
                            });
                            ui.radio_value(
                                &mut config.disk_drives,
                                DiskDrives::One,
                                DiskDrives::One.to_string(),
                            );
                            ui.radio_value(
                                &mut config.disk_drives,
                                DiskDrives::Two,
                                DiskDrives::Two.to_string(),
                            );
                        }
                    });
                });
            });
        });

        ui.add_space(15.0);

        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                ui.label("CRT Controller:");
            });
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if let Some(config) = &mut self.changed_config {
                            ui.radio_value(
                                &mut config.crtc,
                                CrtcType::Type0,
                                CrtcType::Type0.to_string(),
                            );
                            ui.radio_value(
                                &mut config.crtc,
                                CrtcType::Type1,
                                CrtcType::Type1.to_string(),
                            );
                        }
                    });
                    ui.add_space(15.0);
                    ui.vertical(|ui| {
                        if let Some(config) = &mut self.changed_config {
                            ui.radio_value(
                                &mut config.crtc,
                                CrtcType::Type2,
                                CrtcType::Type2.to_string(),
                            );
                            ui.radio_value(
                                &mut config.crtc,
                                CrtcType::Type4,
                                CrtcType::Type4.to_string(),
                            );
                        }
                    });
                });
            });
        });
    }

    fn render_rom_config(&mut self, ui: &mut egui::Ui) {
        self.render_rom_folder(ui);

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.label("Preferred Language:");
            egui::ComboBox::from_id_salt("rom_language_selector")
                .selected_text(self.access_config().preferred_language.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.access_config_mut().preferred_language,
                        RomLanguage::Danish,
                        "Danish",
                    );
                    ui.selectable_value(
                        &mut self.access_config_mut().preferred_language,
                        RomLanguage::English,
                        "English",
                    );
                    ui.selectable_value(
                        &mut self.access_config_mut().preferred_language,
                        RomLanguage::French,
                        "French",
                    );
                    ui.selectable_value(
                        &mut self.access_config_mut().preferred_language,
                        RomLanguage::Spanish,
                        "Spanish",
                    );
                });
        });

        let mut auto_config = true;
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.checkbox(
                &mut auto_config,
                "Automatically configure based on selected hardware model",
            );
        });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Original ROMs:");
        });
        ui.group(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                egui::Grid::new("original_roms_grid")
                    .num_columns(4)
                    .spacing([20.0, 20.0])
                    .show(ui, |ui| {
                        for rom in ORIGINAL_ROMS.iter() {
                            if let Some(RomVariant::Language(language)) = rom.info.variant
                                && language != self.access_config().preferred_language
                            {
                                continue;
                            }
                            ui.vertical(|ui| {
                                let title = match &rom.info.variant {
                                    Some(variant) => {
                                        format!("{} ({})", rom.info.name, variant)
                                    }
                                    None => rom.info.name.to_string(),
                                };
                                ui.add(egui::Label::new(title).extend());
                                ui.add(
                                    egui::Label::new(format!(
                                        "SHA3-256: {}",
                                        hex::encode(&rom.info.hash)
                                    ))
                                    .truncate(),
                                );
                            });

                            let found = false;
                            if found {
                                ui.colored_label(colors::FORREST_GREEN, "found");
                            } else {
                                ui.colored_label(colors::DARK_GRAY, "not found");
                            }

                            if let Some(slot) = rom.info.slot {
                                ui.label(format!("Slot: {}", slot));
                            } else {
                                ui.label("No slot");
                            }

                            let mut used = true;
                            ui.add_enabled(
                                !self.access_config().auto_config,
                                egui::Checkbox::new(&mut used, "Use"),
                            );

                            ui.end_row();
                        }
                    });
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_rom_folder(&mut self, ui: &mut egui::Ui) {
        let mut valid = false;
        ui.horizontal(|ui| {
            ui.label("ROM folder:");
            let (enabled, mut path) = match &self.access_config().rom_folder {
                Some(path_buf) => (true, path_buf.as_os_str().to_string_lossy().to_string()),
                None => (false, "".to_string()),
            };
            ui.add_enabled(enabled, egui::TextEdit::singleline(&mut path));
            self.access_config_mut().rom_folder = Some(PathBuf::from(path));

            valid = match &self.access_config().rom_folder {
                Some(path_buf) => {
                    path_buf.is_dir() && {
                        if let Ok(metadata) = std::fs::metadata(path_buf) {
                            !metadata.permissions().readonly()
                        } else {
                            false
                        }
                    }
                }
                None => false,
            };
            if ui.add_enabled(valid, egui::Button::new("Open")).clicked()
                && let Some(path) = &self.access_config().rom_folder
            {
                open::that(path).unwrap_or_else(|e| {
                    log::error!("Failed to open ROM folder {:?}: {}", path, e);
                });
            }
            if ui
                .button("Pick")
                .on_hover_text("Pick a different folder")
                .clicked()
            {
                pick_folder("ROM Folder", self.picked_rom_folder.clone());
            }
        });
        if !valid {
            ui.horizontal(|ui| {
                ui.colored_label(
                    colors::DARK_RED,
                    "The specified folder does not exist or is not writable.",
                );
                if ui.button("Create").clicked()
                    && let Some(path_buf) = &self.access_config().rom_folder
                {
                    std::fs::create_dir_all(path_buf).unwrap_or_else(|e| {
                        log::error!("Failed to create ROM folder {:?}: {}", path_buf, e);
                    });
                }
            });
        }
    }

    fn handle_picked_rom_folder(&mut self) {
        if let Some(picked_rom_folder) = self.picked_rom_folder.try_with_mut(|f| f.take()).flatten()
        {
            self.access_config_mut().rom_folder = Some(picked_rom_folder);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render_rom_folder(&mut self, ui: &mut egui::Ui) {
        ui.label("ROMs are stored in the browser's IndexedDB.");
        ui.button("Clear ROMs")
            .on_hover_text("Clears all ROMs stored in the browser's IndexedDB.");
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};
    use kittest::NodeT;

    #[test]
    fn test_system_config_modal_opens_and_closes() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig::default();

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Check that the modal heading is actually rendered
        harness.get_by_label("System Configuration");

        // Click OK button to close
        harness.get_by_label("OK").click();
        harness.run();

        // Modal should no longer be visible
        assert!(harness.query_by_label("System Configuration").is_none());
    }

    #[test]
    fn test_disk_drives_validation_by_model() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig {
            model: CpcModel::Cpc464,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Initially None should be selected for CPC 464
        let none_option = harness.get_by_label("None");
        assert_eq!(
            none_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click on CPC 664 radio button
        harness.get_by_label("Amstrad CPC 664").click();
        harness.run();

        // Verify that "Drive A only" is now selected
        let drive_a_option = harness.get_by_label("Drive A only");
        assert_eq!(
            drive_a_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn test_restore_defaults_button() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            crtc: CrtcType::Type4,
            disk_drives: DiskDrives::Two,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Verify initial state - CPC 6128 should be selected
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click the Restore Defaults button
        harness.get_by_label("Restore Defaults").click();
        harness.run();

        // Verify UI shows defaults - CPC 464 should now be selected
        let cpc464_option = harness.get_by_label("Amstrad CPC 464");
        assert_eq!(
            cpc464_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let type0_option = harness.get_by_label("Type 0 (HD6845S/UM6845)");
        assert_eq!(
            type0_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let none_option = harness.get_by_label("None");
        assert_eq!(
            none_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn test_crtc_selection_changes_config() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig::default();

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Initially Type 0 should be selected
        let type0_option = harness.get_by_label("Type 0 (HD6845S/UM6845)");
        assert_eq!(
            type0_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click on Type 2 CRTC option
        harness.get_by_label("Type 2 (MC6845)").click();
        harness.run();

        // Verify Type 2 is now selected in UI
        let type2_option = harness.get_by_label("Type 2 (MC6845)");
        assert_eq!(
            type2_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn test_cancel_button_preserves_config() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            crtc: CrtcType::Type4,
            disk_drives: DiskDrives::Two,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Verify initial UI state - CPC 6128 should be selected
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Make some changes in the UI
        harness.get_by_label("Amstrad CPC 464").click();
        harness.run();
        harness.get_by_label("Type 0 (HD6845S/UM6845)").click();
        harness.run();
        harness.get_by_label("Drive A only").click();
        harness.run();

        // Click Cancel button
        harness.get_by_label("Cancel").click();
        harness.run();

        // Reopen modal to verify original config was preserved
        drop(harness);
        modal.show = true;
        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Verify original values are still selected in UI
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let type4_option = harness.get_by_label("Type 4 (AMS40226)");
        assert_eq!(
            type4_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let drives_ab_option = harness.get_by_label("Drives A and B");
        assert_eq!(
            drives_ab_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }
}
