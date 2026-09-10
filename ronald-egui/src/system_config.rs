use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
#[cfg(target_arch = "wasm32")]
use std::path::Path;
use std::path::PathBuf;

use eframe::egui;
use web_time::Instant;

use ronald_core::system::memory::RomSlot;
pub use ronald_core::system::{CpcModel, CrtcType, DiskDrives, SystemConfig as CoreSystemConfig};
use serde::{Deserialize, Serialize};
use sha3::Digest;

use crate::colors;
use crate::system_config::known_roms::{ORIGINAL_ROMS, enriched_rom_info};
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::files::pick_folder;
use crate::utils::files::{File, download_file, pick_multiple_files};
use crate::utils::sync::{Shared, SharedExt, shared};

mod known_roms;
#[cfg(target_arch = "wasm32")]
mod rom_store;

const SCAN_INTERVAL_SECS: u64 = 3;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SystemConfig {
    model: CpcModel,
    crtc: CrtcType,
    disk_drives: DiskDrives,
    rom_folder: Option<PathBuf>,
    preferred_language: RomLanguage,
    auto_config: bool,
    assigned_roms: Vec<AssignedRom>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for SystemConfig {
    fn default() -> Self {
        let rom_folder = directories::ProjectDirs::from("dev", "int82", "ronald")
            .map(|dirs| dirs.data_dir().join("roms"));

        Self {
            model: CpcModel::Cpc6128,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::Two,
            rom_folder,
            preferred_language: RomLanguage::English,
            auto_config: true,
            assigned_roms: Vec::new(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for SystemConfig {
    fn default() -> Self {
        let rom_folder = None;

        Self {
            model: CpcModel::Cpc6128,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::Two,
            rom_folder,
            preferred_language: RomLanguage::English,
            auto_config: true,
            assigned_roms: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SystemConfigState {
    Unknown,
    Invalid,
    Valid(CoreSystemConfig),
}

impl SystemConfigState {
    pub fn take(&mut self) -> Self {
        if let SystemConfigState::Valid(_) = self {
            std::mem::replace(self, SystemConfigState::Unknown)
        } else {
            self.clone()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_core_config(config: &SystemConfig, result: Shared<SystemConfigState>) {
    if config.assigned_roms.is_empty() {
        result.with_mut(|r| *r = SystemConfigState::Invalid);
        return;
    }

    let SystemConfig {
        model,
        crtc,
        disk_drives,
        ..
    } = *config;

    let mut roms = HashMap::new();
    let mut invalid = false;
    for rom in &config.assigned_roms {
        let image = match std::fs::read(&rom.key.0) {
            Ok(image) => image,
            Err(e) => {
                log::error!(
                    "Failed to read ROM file {:?} for slot {}: {}",
                    rom.key.0,
                    rom.slot,
                    e
                );
                invalid = true;
                break;
            }
        };
        roms.insert(rom.slot, image);
    }

    result.with_mut(|r| {
        if invalid {
            *r = SystemConfigState::Invalid;
        } else {
            *r = SystemConfigState::Valid(CoreSystemConfig {
                model,
                crtc,
                disk_drives,
                roms,
            });
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn build_core_config(config: &SystemConfig, result: Shared<SystemConfigState>) {
    if config.assigned_roms.is_empty() {
        result.with_mut(|r| *r = SystemConfigState::Invalid);
        return;
    }

    let model = config.model;
    let crtc = config.crtc;
    let disk_drives = config.disk_drives;
    let assigned_roms = config.assigned_roms.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let stored = match rom_store::load_roms().await {
            Ok(stored) => stored,
            Err(e) => {
                log::error!("Failed to load ROMs from IndexedDB: {}", e);
                return;
            }
        };

        let images = stored
            .into_iter()
            .map(|rom| (rom.hash, rom.image))
            .collect::<HashMap<_, _>>();

        let mut roms = HashMap::new();
        let mut invalid = false;
        for rom in &assigned_roms {
            let Some(image) = images.get(&rom.key.0).cloned() else {
                log::error!(
                    "ROM image for slot {} is missing (hash {})",
                    rom.slot,
                    hex::encode(&rom.key.0)
                );
                invalid = true;
                break;
            };
            roms.insert(rom.slot, image);
        }

        result.with_mut(|r| {
            if invalid {
                *r = SystemConfigState::Invalid;
            } else {
                *r = SystemConfigState::Valid(CoreSystemConfig {
                    model,
                    crtc,
                    disk_drives,
                    roms,
                });
            }
        });
    });
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct RomKey(PathBuf);

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct RomKey(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct AvailableRom {
    key: RomKey,
    info: RomInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct AssignedRom {
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

#[derive(Debug, Default, Clone, Copy)]
enum Tab {
    #[default]
    Hardware,
    Rom,
}

#[derive(Debug)]
enum Command {
    AssignRom { rom: AssignedRom, any_slot: bool },
    UnassignRom { slot: RomSlot },
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Default)]
enum ScanState {
    #[default]
    Idle,
    Scanning,
    Ready(Vec<rom_store::StoredRom>),
}

#[derive(Debug)]
pub struct SystemConfigModal {
    pub show: bool,
    tab: Tab,
    changed_config: Option<SystemConfig>,
    picked_rom_folder: Shared<Option<PathBuf>>,
    picked_import_roms: Shared<Vec<File>>,
    download_url: String,
    downloaded_rom: Shared<Option<File>>,
    last_scan: Instant,
    #[cfg(target_arch = "wasm32")]
    scan_state: Shared<ScanState>,
    available_roms: Vec<AvailableRom>,
    custom_roms: Vec<AvailableRom>,
    pending_commands: Vec<Command>,
    reassign_pending: Option<(AssignedRom, bool)>,
}

impl Default for SystemConfigModal {
    fn default() -> Self {
        Self {
            show: false,
            tab: Tab::Rom,
            changed_config: None,
            picked_rom_folder: shared(None),
            picked_import_roms: shared(Vec::new()),
            download_url: option_env!("RONALD_ROM_URL").unwrap_or("").to_string(),
            downloaded_rom: shared(None),
            last_scan: Instant::now(),
            #[cfg(target_arch = "wasm32")]
            scan_state: shared(ScanState::Idle),
            available_roms: Vec::new(),
            custom_roms: Vec::new(),
            pending_commands: Vec::new(),
            reassign_pending: None,
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
            self.update_available_roms(true);
        }

        self.handle_picked_rom_folder();
        self.handle_rom_import();

        egui::Modal::new("system_config_modal".into()).show(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(10.0);
                ui.heading("System Configuration");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(matches!(self.tab, Tab::Rom), "System ROMs")
                        .clicked()
                    {
                        self.tab = Tab::Rom;
                    }
                    if ui
                        .selectable_label(matches!(self.tab, Tab::Hardware), "Hardware")
                        .clicked()
                    {
                        self.tab = Tab::Hardware;
                    }
                });
                ui.separator();

                match self.tab {
                    Tab::Hardware => self.render_hardware_config(ui),
                    Tab::Rom => self.render_rom_config(ui),
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() {
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

        self.handle_commands();

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
                        let mut model = self.access_config().model;

                        ui.radio_value(&mut model, CpcModel::Cpc464, "Amstrad CPC 464");
                        ui.radio_value(&mut model, CpcModel::Cpc664, "Amstrad CPC 664");
                        ui.radio_value(&mut model, CpcModel::Cpc6128, "Amstrad CPC 6128");

                        if model != self.access_config().model {
                            self.unapply_auto_config();
                            self.access_config_mut().model = model;
                            self.apply_auto_config();
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
                        let mut disk_drives = self.access_config().disk_drives;

                        let none_enabled = self.access_config().model == CpcModel::Cpc464;
                        // Ensure we don't have None selected for models that require at least one drive
                        if !none_enabled && disk_drives == DiskDrives::None {
                            disk_drives = DiskDrives::One;
                        }

                        ui.add_enabled_ui(none_enabled, |ui| {
                            ui.radio_value(
                                &mut disk_drives,
                                DiskDrives::None,
                                DiskDrives::None.to_string(),
                            );
                        });
                        ui.radio_value(
                            &mut disk_drives,
                            DiskDrives::One,
                            DiskDrives::One.to_string(),
                        );
                        ui.radio_value(
                            &mut disk_drives,
                            DiskDrives::Two,
                            DiskDrives::Two.to_string(),
                        );

                        if disk_drives != self.access_config().disk_drives {
                            self.unapply_auto_config();
                            self.access_config_mut().disk_drives = disk_drives;
                            self.apply_auto_config();
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
        self.update_available_roms(false);

        if self.access_config().assigned_roms.is_empty() {
            ui.colored_label(
                colors::DARK_RED,
                "No system ROMs are assigned. The emulator will not start.",
            );
            ui.add_space(15.0);
        }

        self.render_rom_folder(ui);
        self.render_rom_import(ui);
        self.render_reassign_modal(ui);

        ui.add_space(15.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            if ui
                .checkbox(
                    &mut self.access_config_mut().auto_config,
                    "Automatically configure based on selected hardware model",
                )
                .clicked()
                && self.access_config().auto_config
            {
                self.apply_auto_config();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Preferred Language:");
            egui::ComboBox::from_id_salt("rom_language_selector")
                .selected_text(self.access_config().preferred_language.to_string())
                .show_ui(ui, |ui| {
                    let mut preferred_language = self.access_config_mut().preferred_language;
                    if ui
                        .selectable_value(&mut preferred_language, RomLanguage::Danish, "Danish")
                        .clicked()
                    {
                        self.unapply_auto_config();
                        self.access_config_mut().preferred_language = preferred_language;
                        self.apply_auto_config();
                    }
                    if ui
                        .selectable_value(&mut preferred_language, RomLanguage::English, "English")
                        .clicked()
                    {
                        self.unapply_auto_config();
                        self.access_config_mut().preferred_language = preferred_language;
                        self.apply_auto_config();
                    }
                    if ui
                        .selectable_value(&mut preferred_language, RomLanguage::French, "French")
                        .clicked()
                    {
                        self.unapply_auto_config();
                        self.access_config_mut().preferred_language = preferred_language;
                        self.apply_auto_config();
                    }
                    if ui
                        .selectable_value(&mut preferred_language, RomLanguage::Spanish, "Spanish")
                        .clicked()
                    {
                        self.unapply_auto_config();
                        self.access_config_mut().preferred_language = preferred_language;
                        self.apply_auto_config();
                    }
                });
        });

        ui.separator();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Original ROMs:");
        });
        ui.group(|ui| {
            egui::ScrollArea::vertical()
                .id_salt("original_roms_scroll_area")
                .max_height(400.0)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        egui::Grid::new("required_original_roms_grid")
                            .num_columns(3)
                            .spacing([20.0, 20.0])
                            .show(ui, |ui| {
                                ui.set_min_width(600.0);
                                for original_rom in ORIGINAL_ROMS.iter() {
                                    if !self.required_by_auto_config(original_rom) {
                                        continue;
                                    }
                                    self.render_original_rom(ui, original_rom, false);
                                }
                            });

                        ui.add_space(20.0);

                        egui::CollapsingHeader::new("Other detected original ROMs")
                            .show_unindented(ui, |ui| {
                                egui::Grid::new("other_original_roms_grid")
                                    .num_columns(3)
                                    .spacing([20.0, 20.0])
                                    .show(ui, |ui| {
                                        ui.set_min_width(600.0);
                                        for original_rom in ORIGINAL_ROMS.iter() {
                                            if self.required_by_auto_config(original_rom) {
                                                continue;
                                            }
                                            self.render_original_rom(ui, original_rom, true);
                                        }
                                    });
                            });
                    });
                });
        });

        if self.custom_roms.is_empty() {
            return;
        }

        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Custom ROMs:");
        });
        ui.group(|ui| {
            egui::ScrollArea::vertical()
                .id_salt("custom_roms_scroll_area")
                .max_height(400.0)
                .show(ui, |ui| {
                    egui::Grid::new("custom_roms_grid")
                        .num_columns(3)
                        .spacing([20.0, 20.0])
                        .show(ui, |ui| {
                            ui.set_min_width(600.0);
                            let custom_roms = std::mem::take(&mut self.custom_roms);
                            for custom_rom in &custom_roms {
                                self.render_custom_rom(ui, custom_rom);
                            }
                            self.custom_roms = custom_roms;
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
            let response = ui.add_enabled(enabled, egui::TextEdit::singleline(&mut path));
            if response.changed() {
                self.access_config_mut().rom_folder = Some(PathBuf::from(path));
                self.update_available_roms(true);
            }

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

    #[cfg(target_arch = "wasm32")]
    fn render_rom_folder(&mut self, ui: &mut egui::Ui) {
        ui.label("ROMs are stored in the browser's IndexedDB.");
    }

    fn handle_picked_rom_folder(&mut self) {
        if let Some(picked_rom_folder) = self.picked_rom_folder.try_with_mut(|f| f.take()).flatten()
        {
            self.access_config_mut().rom_folder = Some(picked_rom_folder);
            self.update_available_roms(true);
        }
    }

    fn render_rom_import(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        egui::Grid::new("rom_import_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Import local ROMs:");
                if ui.button("Select Files").clicked() {
                    pick_multiple_files(
                        "Select ROMs to import",
                        "ROM Files",
                        &["rom", "zip"],
                        self.picked_import_roms.clone(),
                    );
                }
                ui.end_row();

                ui.label("Import from URL:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.download_url);
                    if ui.button("Download").clicked() {
                        download_file(&self.download_url, self.downloaded_rom.clone());
                    }
                });
                ui.end_row();
            });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn handle_rom_import(&mut self) {
        let mut refresh = false;
        self.picked_import_roms.try_with_mut(|files| {
            let mut files = std::mem::take(files);

            if let Some(downloaded_rom) = self.downloaded_rom.try_with_mut(|f| f.take()).flatten() {
                files.push(downloaded_rom);
            }

            if files.is_empty() {
                return;
            }

            if self.access_config().rom_folder.is_none() {
                log::warn!("ROM folder is not set, cannot import ROMs");
                return;
            };

            refresh = true;
            for file in files {
                let Ok(mut archive) = zip::ZipArchive::new(std::io::Cursor::new(&file.image))
                else {
                    // Not a ZIP archive, treat it as a single ROM file
                    self.import_rom_file(&file, false);
                    continue;
                };

                for i in 0..archive.len() {
                    if let Ok(mut file) = archive.by_index(i) {
                        if file.is_dir() {
                            continue;
                        }

                        if !file.name().to_lowercase().ends_with(".rom") {
                            log::info!("Skipping non-ROM file in ZIP: {}", file.name());
                            continue;
                        }

                        let mut image = Vec::new();
                        if file.read_to_end(&mut image).is_ok() {
                            let rom_file = File {
                                path_buf: PathBuf::from(file.name()),
                                image,
                            };

                            self.import_rom_file(&rom_file, true);
                        } else {
                            log::warn!("Failed to read ROM file from ZIP: {}", file.name());
                        }
                    }
                }
            }
        });

        if refresh {
            self.update_available_roms(true);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn import_rom_file(&self, file: &File, preserve_path: bool) {
        let Some(rom_folder) = self.access_config().rom_folder.as_ref() else {
            return;
        };

        let destination = if preserve_path {
            rom_folder.join(&file.path_buf)
        } else {
            rom_folder.join(
                file.path_buf
                    .file_name()
                    .expect("ROM file should have a file name"),
            )
        };

        if let Some(parent) = destination.parent()
            && parent != rom_folder
        {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| {
                log::error!(
                    "Failed to create directories for ROM file {:?}: {}",
                    file.path_buf,
                    err
                );
            });
        }

        std::fs::write(&destination, &file.image).unwrap_or_else(|err| {
            log::error!(
                "Failed to write ROM file {:?} to ROM folder: {}",
                file.path_buf,
                err
            );
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_rom_import(&mut self) {
        let mut roms = Vec::new();

        self.picked_import_roms.try_with_mut(|files| {
            let mut files = std::mem::take(files);

            if let Some(downloaded_rom) = self.downloaded_rom.try_with_mut(|f| f.take()).flatten() {
                files.push(downloaded_rom);
            }

            for file in files {
                let Ok(mut archive) = zip::ZipArchive::new(std::io::Cursor::new(&file.image))
                else {
                    // Not a ZIP archive, treat it as a single ROM file
                    roms.extend(self.import_rom_file(&file, false));
                    continue;
                };

                for i in 0..archive.len() {
                    if let Ok(mut file) = archive.by_index(i) {
                        if file.is_dir() {
                            continue;
                        }

                        if !file.name().to_lowercase().ends_with(".rom") {
                            log::info!("Skipping non-ROM file in ZIP: {}", file.name());
                            continue;
                        }

                        let mut image = Vec::new();
                        if file.read_to_end(&mut image).is_ok() {
                            let rom_file = File {
                                path_buf: PathBuf::from(file.name()),
                                image,
                            };

                            roms.extend(self.import_rom_file(&rom_file, true));
                        } else {
                            log::warn!("Failed to read ROM file from ZIP: {}", file.name());
                        }
                    }
                }
            }
        });

        if !roms.is_empty() {
            // Two separate transactions are created by store_imported_roms() and
            // update_available_roms() and IndexedDB guarantees their order.
            self.store_imported_roms(roms);
            self.update_available_roms(true);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn import_rom_file(&self, file: &File, preserve_path: bool) -> Option<rom_store::StoredRom> {
        let path = if preserve_path {
            file.path_buf.as_path()
        } else {
            Path::new(
                file.path_buf
                    .file_name()
                    .expect("ROM file should have a file name"),
            )
        };
        let hash = sha3::Sha3_256::digest(&file.image).to_vec();

        Some(rom_store::StoredRom {
            hash,
            name: path.to_string_lossy().to_string(),
            image: file.image.clone(),
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn store_imported_roms(&self, roms: Vec<rom_store::StoredRom>) {
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = rom_store::store_roms(&roms).await {
                log::error!("Failed to store imported ROMs in IndexedDB: {}", e);
            }
        });
    }

    fn render_original_rom(
        &mut self,
        ui: &mut egui::Ui,
        original_rom: &OriginalRom,
        skip_missing: bool,
    ) {
        let available_rom = self
            .available_roms
            .iter()
            .find(|r| r.info.hash == original_rom.info.hash);

        if skip_missing && available_rom.is_none() {
            return;
        }

        let was_used = match available_rom {
            Some(rom) => self
                .access_config()
                .assigned_roms
                .iter()
                .any(|r| r.key == rom.key),
            None => false,
        };

        ui.vertical(|ui| {
            let title = match &original_rom.info.variant {
                Some(variant) => {
                    format!("{} ({})", original_rom.info.name, variant)
                }
                None => original_rom.info.name.to_string(),
            };
            ui.add(egui::Label::new(title).extend());
            ui.add(
                egui::Label::new(format!(
                    "SHA3-256: {}",
                    hex::encode(&original_rom.info.hash)
                ))
                .truncate(),
            );
        });

        if let Some(slot) = original_rom.info.slot {
            ui.label(format!("Slot: {}", slot));
        } else {
            ui.label("Any slot");
        }

        if let Some(rom) = available_rom {
            let reassign_pending = self
                .reassign_pending
                .as_ref()
                .is_some_and(|r| r.0.key == rom.key);

            let mut used = was_used || reassign_pending;
            if ui
                .add_enabled(
                    !self.access_config().auto_config,
                    egui::Checkbox::new(&mut used, "Use"),
                )
                .clicked()
            {
                let slot = rom.info.slot.expect("original ROMs should have a slot");

                if used {
                    let key = rom.key.clone();

                    self.pending_commands.push(Command::AssignRom {
                        rom: AssignedRom { key, slot },
                        any_slot: false,
                    });
                } else {
                    self.pending_commands.push(Command::UnassignRom { slot });
                }
            }
        } else {
            ui.colored_label(colors::DARK_RED, "not found");
        }

        ui.end_row();
    }

    fn render_custom_rom(&mut self, ui: &mut egui::Ui, custom_rom: &AvailableRom) {
        let assigned_slot = self
            .access_config()
            .assigned_roms
            .iter()
            .find(|r| r.key == custom_rom.key)
            .map(|r| r.slot);

        let was_used = assigned_slot.is_some();

        ui.vertical(|ui| {
            let title = match &custom_rom.info.variant {
                Some(variant) => {
                    format!("{} ({})", custom_rom.info.name, variant)
                }
                None => custom_rom.info.name.to_string(),
            };
            ui.add(egui::Label::new(title).extend());
            ui.add(
                egui::Label::new(format!("SHA3-256: {}", hex::encode(&custom_rom.info.hash)))
                    .truncate(),
            );
        });

        if let Some(slot) = custom_rom.info.slot.or(assigned_slot) {
            ui.label(format!("Slot: {}", slot));
        } else {
            ui.label("Any slot");
        }

        let reassign_pending = self
            .reassign_pending
            .as_ref()
            .is_some_and(|r| r.0.key == custom_rom.key);

        let mut used = was_used || reassign_pending;
        if ui.checkbox(&mut used, "Use").clicked() {
            if used {
                let key = custom_rom.key.clone();

                match custom_rom.info.slot {
                    Some(slot) => {
                        self.pending_commands.push(Command::AssignRom {
                            rom: AssignedRom { key, slot },
                            any_slot: false,
                        });
                    }
                    None => {
                        let slot = RomSlot::Lower;
                        self.reassign_pending = Some((AssignedRom { key, slot }, true));
                    }
                }
            } else {
                let slot = assigned_slot.expect("assigned custom ROM should have slot");
                self.pending_commands.push(Command::UnassignRom { slot });
            }
        }

        ui.end_row();
    }

    fn render_reassign_modal(&mut self, ui: &mut egui::Ui) {
        let Some((AssignedRom { key: _, slot }, any_slot)) = self.reassign_pending else {
            return;
        };

        egui::Modal::new("reassign_slot".into()).show(ui, |ui| {
            ui.set_min_width(400.0);


            if any_slot {
                ui.label("Select a slot for the ROM:");

                if ui
                    .radio(matches!(slot, RomSlot::Lower), "Lower ROM")
                    .clicked()
                    && let Some((AssignedRom { key, slot: _ }, _)) = self.reassign_pending.take()
                {
                    let slot = RomSlot::Lower;
                    self.reassign_pending = Some((AssignedRom { key, slot }, true));
                };

                ui.horizontal(|ui| {
                    if ui
                        .radio(matches!(slot, RomSlot::Upper(_)), "Upper ROM")
                        .clicked()
                        && let Some((AssignedRom { key, slot: _ }, _)) =
                            self.reassign_pending.take()
                    {
                        let slot = RomSlot::Upper(0);
                        self.reassign_pending = Some((AssignedRom { key, slot }, true));
                    };

                    ui.add_enabled_ui(matches!(slot, RomSlot::Upper(_)), |ui| {
                        let mut slot_number = match slot {
                            RomSlot::Lower => 0,
                            RomSlot::Upper(n) => n,
                        };
                        if ui
                            .add(egui::DragValue::new(&mut slot_number).range(0..=255))
                            .changed()
                            && let Some((AssignedRom { key, slot: _ }, _)) =
                                self.reassign_pending.take()
                        {
                            let slot = RomSlot::Upper(slot_number);
                            self.reassign_pending = Some((AssignedRom { key, slot }, true));
                        }
                    });
                });
            }

            let assigned_rom = self
                .access_config()
                .assigned_roms
                .iter()
                .find(|r| r.slot == slot);
            let mut required_by_auto_config = false;
            if assigned_rom.is_some() {
                required_by_auto_config = assigned_rom
                    .and_then(|assigned| {
                        self
                            .available_roms
                            .iter()
                            .find(|available| available.key == assigned.key) })
                    .and_then(|available| {
                        ORIGINAL_ROMS
                            .iter()
                            .find(|original| original.info.hash == available.info.hash) })
                    .is_some_and(|original| self.required_by_auto_config(original));


                ui.add_space(20.0);

                if required_by_auto_config {
                    ui.label(format!(
                        "Slot \"{}\" is occupied by a ROM required by auto config. Disable auto config and reassign this slot?",
                        slot
                    ));
                } else {
                    ui.label(format!("Slot \"{}\" is already occupied. Reassign?", slot));
                }
            }

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("Ok").clicked() {
                    if required_by_auto_config {
                        self.access_config_mut().auto_config = false;
                    }

                    let (rom, _) = self
                        .reassign_pending
                        .take()
                        .expect("reassign_pending should be Some");

                    self.pending_commands.push(Command::UnassignRom { slot });
                    self.pending_commands
                        .push(Command::AssignRom { rom, any_slot });
                }
                if ui.button("Cancel").clicked() {
                    self.reassign_pending = None;
                }
            });
        });
    }

    fn update_available_roms(&mut self, force: bool) {
        let roms_changed = self.scan_available_roms(force);
        if force || roms_changed {
            self.filter_custom_roms();
            self.apply_auto_config();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn scan_available_roms(&mut self, force: bool) -> bool {
        if !force && self.last_scan.elapsed().as_secs() <= SCAN_INTERVAL_SECS {
            return false;
        }

        self.last_scan = Instant::now();
        let mut roms = Vec::with_capacity(256);
        if self.access_config().rom_folder.is_none() {
            return true;
        }

        for rom in walkdir::WalkDir::new(self.access_config().rom_folder.as_ref().unwrap()) {
            let Ok(entry) = rom else {
                continue;
            };

            if !entry.file_type().is_file() {
                continue;
            }

            match entry.metadata() {
                Ok(metadata) => {
                    if metadata.len() != 16 * 1024 {
                        continue; // Skip files that are not 16KB
                    }
                }
                Err(_) => {
                    log::warn!("Failed to get metadata for file: {:?}", entry.path());
                    continue;
                }
            }

            let contents = match std::fs::read(entry.path()) {
                Ok(contents) => contents,
                Err(e) => {
                    log::error!("Failed to read file {:?}: {}", entry.path(), e);
                    continue;
                }
            };

            let name = entry
                .path()
                .file_name()
                .unwrap_or(std::ffi::OsStr::new("unknown"))
                .to_string_lossy()
                .to_string();
            let hash = sha3::Sha3_256::digest(&contents).to_vec();

            roms.push(AvailableRom {
                key: RomKey(entry.path().to_path_buf()),
                info: enriched_rom_info(name, hash),
            });
        }

        roms.dedup_by(|a, b| a.info.hash == b.info.hash);
        roms.sort_by(|a, b| a.info.name.cmp(&b.info.name));
        let changed = roms != self.available_roms;

        self.access_config_mut()
            .assigned_roms
            .retain(|assigned| roms.iter().any(|available| available.key == assigned.key));

        self.available_roms = roms;
        changed
    }

    #[cfg(target_arch = "wasm32")]
    fn scan_available_roms(&mut self, force: bool) -> bool {
        let scanned = self
            .scan_state
            .try_with_mut(|state| match std::mem::replace(state, ScanState::Idle) {
                ScanState::Ready(stored) => Some(stored),
                other => {
                    *state = other;
                    None
                }
            })
            .flatten();

        let Some(stored) = scanned else {
            if force || self.last_scan.elapsed().as_secs() > SCAN_INTERVAL_SECS {
                self.last_scan = Instant::now();
                self.start_scan();
            }

            return false;
        };
        self.last_scan = Instant::now();

        let mut roms = stored
            .into_iter()
            .filter(|stored| stored.image.len() == 16 * 1024) // Skip files that are not 16KB
            .map(|stored| AvailableRom {
                key: RomKey(stored.hash.clone()),
                info: enriched_rom_info(stored.name, stored.hash),
            })
            .collect::<Vec<_>>();

        roms.dedup_by(|a, b| a.info.hash == b.info.hash);
        roms.sort_by(|a, b| a.info.name.cmp(&b.info.name));
        let changed = roms != self.available_roms;

        self.access_config_mut()
            .assigned_roms
            .retain(|assigned| roms.iter().any(|available| available.key == assigned.key));

        self.available_roms = roms;
        changed
    }

    #[cfg(target_arch = "wasm32")]
    fn start_scan(&self) {
        let state = self.scan_state.clone();
        let busy = state
            .try_with_mut(|state| match state {
                ScanState::Idle => {
                    *state = ScanState::Scanning;
                    false
                }
                _ => true,
            })
            .unwrap_or(true);
        if busy {
            return;
        }

        wasm_bindgen_futures::spawn_local(async move {
            let stored = rom_store::load_roms().await.unwrap_or_else(|e| {
                log::error!("Failed to scan ROMs from IndexedDB: {}", e);
                Vec::new()
            });
            state.with_mut(|state| *state = ScanState::Ready(stored));
        });
    }

    fn filter_custom_roms(&mut self) {
        let original_rom_hashes = ORIGINAL_ROMS
            .iter()
            .map(|original| original.info.hash.clone())
            .collect::<Vec<_>>();

        self.custom_roms = self
            .available_roms
            .iter()
            .filter(|rom| !original_rom_hashes.contains(&rom.info.hash))
            .cloned()
            .collect();
    }

    fn required_by_auto_config(&self, original_rom: &OriginalRom) -> bool {
        let model = &self.access_config().model;
        if !original_rom.auto_config_rule.models.contains(model) {
            return false;
        }

        let has_disk_drive = !matches!(self.access_config().disk_drives, DiskDrives::None);
        if original_rom.auto_config_rule.requires_disk_drive && !has_disk_drive {
            return false;
        }

        let preferred_language_exists = || {
            ORIGINAL_ROMS.iter().any(|r| {
                r.info.slot == original_rom.info.slot
                    && r.info.variant
                        == Some(RomVariant::Language(
                            self.access_config().preferred_language,
                        ))
                    && r.auto_config_rule.models.contains(model)
            })
        };

        match &original_rom.info.variant {
            Some(RomVariant::Language(language)) => {
                *language == self.access_config().preferred_language
                    || (*language == RomLanguage::English && !preferred_language_exists())
            }
            _ => true,
        }
    }

    fn apply_auto_config(&mut self) {
        if !self.access_config().auto_config {
            return;
        }

        for original_rom in ORIGINAL_ROMS.iter() {
            if !self.required_by_auto_config(original_rom) {
                continue;
            }

            let Some(available_rom) = self
                .available_roms
                .iter()
                .find(|r| r.info.hash == original_rom.info.hash)
            else {
                continue;
            };

            let key = available_rom.key.clone();
            let slot = original_rom
                .info
                .slot
                .expect("original ROMs should have a slot");

            self.access_config_mut()
                .assigned_roms
                .retain(|r| r.slot != slot);

            self.access_config_mut()
                .assigned_roms
                .push(AssignedRom { key, slot });
        }
    }

    fn unapply_auto_config(&mut self) {
        for original in ORIGINAL_ROMS.iter() {
            if !self.required_by_auto_config(original) {
                continue;
            }

            let key = self
                .available_roms
                .iter()
                .find(|available| available.info.hash == original.info.hash)
                .map(|available| available.key.clone());

            self.access_config_mut()
                .assigned_roms
                .retain(|assigned| key.as_ref().is_some_and(|key| *key != assigned.key));
        }
    }

    fn handle_commands(&mut self) {
        for command in std::mem::take(&mut self.pending_commands) {
            match command {
                Command::AssignRom { rom, any_slot } => {
                    if self
                        .access_config()
                        .assigned_roms
                        .iter()
                        .any(|r| r.slot == rom.slot && r.key != rom.key)
                    {
                        self.reassign_pending = Some((rom, any_slot));
                        continue;
                    }
                    self.access_config_mut()
                        .assigned_roms
                        .retain(|r| r.slot != rom.slot);
                    self.access_config_mut().assigned_roms.push(rom);
                }
                Command::UnassignRom { slot } => {
                    self.access_config_mut()
                        .assigned_roms
                        .retain(|r| r.slot != slot);
                }
            }
        }
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};
    use kittest::NodeT;

    fn open_hardware_tab(harness: &mut Harness<'_>) {
        harness.get_by_label("Hardware").click();
        harness.run();
    }

    fn open_rom_tab(harness: &mut Harness<'_>) {
        harness.get_by_label("System ROMs").click();
        harness.run();
    }

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

        // Click Ok button to close
        harness.get_by_label("Ok").click();
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
        open_hardware_tab(&mut harness);

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
            model: CpcModel::Cpc464,
            crtc: CrtcType::Type4,
            disk_drives: DiskDrives::None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();
        open_hardware_tab(&mut harness);

        // Verify initial state - CPC 464 should be selected
        let cpc464_option = harness.get_by_label("Amstrad CPC 464");
        assert_eq!(
            cpc464_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click the Restore Defaults button
        harness.get_by_label("Restore Defaults").click();
        harness.run();

        // Verify UI shows defaults - CPC 6128 should now be selected
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let type0_option = harness.get_by_label("Type 0 (HD6845S/UM6845)");
        assert_eq!(
            type0_option.accesskit_node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let drives_ab_option = harness.get_by_label("Drives A and B");
        assert_eq!(
            drives_ab_option.accesskit_node().toggled(),
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
        open_hardware_tab(&mut harness);

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
        open_hardware_tab(&mut harness);

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
        open_hardware_tab(&mut harness);

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

    #[test]
    fn test_rom_tab_warns_while_no_roms_are_assigned() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig {
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("No system ROMs are assigned")
                )
                .is_some()
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_rom_folder_warns_when_folder_does_not_exist() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };
        let mut config = SystemConfig {
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let open_button = harness.get_by_label("Open");
        assert!(open_button.accesskit_node().is_disabled());

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("does not exist or is not writable")
                )
                .is_some()
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_rom_folder_create_button_creates_the_folder() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let rom_folder =
            std::env::temp_dir().join("ronald_test_rom_folder_create_button_creates_the_folder");
        let _ = std::fs::remove_dir_all(&rom_folder);

        let mut config = SystemConfig {
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness.get_by_label("Create").click();
        harness.run();

        assert!(rom_folder.exists());
        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_picked_rom_folder_is_applied_to_the_config() {
        let rom_folder =
            std::env::temp_dir().join("ronald_test_picked_rom_folder_is_applied_to_the_config");
        let mut modal = SystemConfigModal {
            show: true,
            picked_rom_folder: shared(Some(rom_folder.clone())),
            ..Default::default()
        };
        let mut config = SystemConfig {
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        drop(harness);
        assert_eq!(modal.access_config().rom_folder, Some(rom_folder));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_local_rom_import_copies_the_file_into_the_rom_folder() {
        let mut modal = SystemConfigModal {
            show: true,
            picked_import_roms: shared(vec![File {
                path_buf: PathBuf::from("test.rom"),
                image: vec![0u8; 16 * 1024],
            }]),
            ..Default::default()
        };

        let rom_folder = std::env::temp_dir()
            .join("ronald_test_local_rom_import_copies_the_file_into_the_rom_folder");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let mut config = SystemConfig {
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(rom_folder.join("test.rom").exists());

        drop(harness);
        assert!(modal.available_roms.len() == 1);
        assert!(modal.available_roms[0].info.name == "test.rom");

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_zip_import_extracts_the_contained_rom_files() {
        let image = {
            use std::io::Write;

            let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
            let options = zip::write::SimpleFileOptions::default();
            zip.add_directory("subdir/", options).unwrap();
            zip.start_file("subdir/test.rom", options).unwrap();
            zip.write_all(&vec![0u8; 16 * 1024]).unwrap();
            zip.finish().unwrap().into_inner()
        };

        let mut modal = SystemConfigModal {
            show: true,
            picked_import_roms: shared(vec![File {
                path_buf: PathBuf::from("test.zip"),
                image,
            }]),
            ..Default::default()
        };

        let rom_folder =
            std::env::temp_dir().join("ronald_test_zip_import_extracts_the_contained_rom_files");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let mut config = SystemConfig {
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(rom_folder.join("subdir/test.rom").exists());

        drop(harness);
        assert!(modal.available_roms.len() == 1);
        assert!(modal.available_roms[0].info.name == "test.rom");

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_import_skips_files_that_are_not_16k() {
        let mut modal = SystemConfigModal {
            show: true,
            picked_import_roms: shared(vec![File {
                path_buf: PathBuf::from("test.rom"),
                image: vec![0u8; 16 * 1024 - 1],
            }]),
            ..Default::default()
        };

        let rom_folder =
            std::env::temp_dir().join("ronald_test_import_skips_files_that_are_not_16k");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let mut config = SystemConfig {
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // If files are not 16KB they are copied, ...
        assert!(rom_folder.join("test.rom").exists());

        drop(harness);
        // ...but not picked up when scanning for available ROMs.
        assert!(modal.available_roms.is_empty());

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_import_skips_roms_that_are_already_available() {
        let mut modal = SystemConfigModal {
            show: true,
            picked_import_roms: shared(vec![
                File {
                    path_buf: PathBuf::from("test.rom"),
                    image: vec![0u8; 16 * 1024],
                },
                File {
                    path_buf: PathBuf::from("duplicate.rom"),
                    image: vec![0u8; 16 * 1024],
                },
            ]),
            ..Default::default()
        };

        let rom_folder =
            std::env::temp_dir().join("ronald_test_import_skips_roms_that_are_already_available");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let mut config = SystemConfig {
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        // Duplicates are copies, ...
        assert!(rom_folder.join("test.rom").exists());
        assert!(rom_folder.join("duplicate.rom").exists());

        drop(harness);
        // ...but only one of them is picked up when scanning for available ROMs.
        assert!(modal.available_roms.len() == 1);

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_downloaded_rom_is_imported() {
        let mut modal = SystemConfigModal {
            show: true,
            downloaded_rom: shared(Some(File {
                path_buf: PathBuf::from("test.rom"),
                image: vec![0u8; 16 * 1024],
            })),
            ..Default::default()
        };

        let rom_folder = std::env::temp_dir().join("ronald_test_downloaded_rom_is_imported");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let mut config = SystemConfig {
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(rom_folder.join("test.rom").exists());

        drop(harness);
        assert!(modal.available_roms.len() == 1);
        assert!(modal.available_roms[0].info.name == "test.rom");

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    fn test_required_original_roms_are_listed_for_the_cpc_464_without_disk() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc464,
            disk_drives: DiskDrives::None,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let required_roms = [
            "b57872c97d569a1968816fcaf359de4b6bf5c188e7dbd0954761cc552b25d327", // 464 OS
            "ff6fbb6e12808e7d32c9217813d730df7321b4b5e499a45e3eac21b421dcf729", // BASIC 1.0
        ];

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("SHA3-256")
                )
                .count(),
            required_roms.len()
        );

        for hash in required_roms.iter() {
            assert!(
                harness
                    .query(
                        kittest::by()
                            .role(egui::accesskit::Role::Label)
                            .label_contains(hash),
                    )
                    .is_some(),
                "missing required ROM with hash {}",
                hash
            )
        }
    }

    #[test]
    fn test_required_original_roms_are_listed_for_the_cpc_464_with_disk() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc464,
            disk_drives: DiskDrives::One,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let required_roms = [
            "b57872c97d569a1968816fcaf359de4b6bf5c188e7dbd0954761cc552b25d327", // 464 OS
            "ff6fbb6e12808e7d32c9217813d730df7321b4b5e499a45e3eac21b421dcf729", // BASIC 1.0
            "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7", // AMSDOS
        ];

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("SHA3-256")
                )
                .count(),
            required_roms.len()
        );

        for hash in required_roms.iter() {
            assert!(
                harness
                    .query(
                        kittest::by()
                            .role(egui::accesskit::Role::Label)
                            .label_contains(hash),
                    )
                    .is_some(),
                "missing required ROM with hash {}",
                hash
            )
        }
    }

    #[test]
    fn test_required_original_roms_are_listed_for_the_cpc_664() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc664,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let required_roms = [
            "31e57547f5405e6216345a7d6d059791208ce62e483ec7d023c7b6bcc80981f9", // 664 OS
            "1260317b7b631d36bccc8b08c93b0f99a530bddc2ff21e858d71224b4d7dab7c", // BASIC 1.1
            "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7", // AMSDOS
        ];

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("SHA3-256")
                )
                .count(),
            required_roms.len()
        );

        for hash in required_roms.iter() {
            assert!(
                harness
                    .query(
                        kittest::by()
                            .role(egui::accesskit::Role::Label)
                            .label_contains(hash),
                    )
                    .is_some(),
                "missing required ROM with hash {}",
                hash
            )
        }
    }

    #[test]
    fn test_required_original_roms_are_listed_for_the_cpc_6128() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let required_roms = [
            "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520", // 6128 OS
            "7630682a0fd8deaa4514954c1a256224d3ae4cb6e3dd4da8e5ad8e1c5c55adc7", // BASIC 1.1
            "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7", // AMSDOS
        ];

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("SHA3-256")
                )
                .count(),
            required_roms.len()
        );

        for hash in required_roms.iter() {
            assert!(
                harness
                    .query(
                        kittest::by()
                            .role(egui::accesskit::Role::Label)
                            .label_contains(hash),
                    )
                    .is_some(),
                "missing required ROM with hash {}",
                hash
            )
        }
    }

    #[test]
    fn test_missing_original_rom_is_marked_as_not_found() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("not found")
                )
                .count(),
            3
        );

        assert_eq!(harness.query_all_by_label("Use").count(), 0);
    }

    #[test]
    fn test_other_detected_original_roms_lists_only_available_roms() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("os6128_french.rom")),
            info: RomInfo::from((
                "CPC 6128 OS",
                "bf87e68ff847052fecc98f84f286b23edc49d607ea8b140fc93e90259cc98ffb",
                Some(RomVariant::Language(RomLanguage::French)),
                Some(RomSlot::Lower),
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("CPC 6128 OS (French)")
                )
                .is_none(),
            "other detected original ROMs should be hidden"
        );

        harness.get_by_label("Other detected original ROMs").click();
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("CPC 6128 OS (French)")
                )
                .is_some(),
            "detected original ROM should be listed"
        );

        assert_eq!(harness.query_all_by_label("Use").count(), 1);
    }

    #[test]
    fn test_auto_config_assigns_the_required_original_roms() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                    Some(RomVariant::Language(RomLanguage::English)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("basic.rom")),
                info: RomInfo::from((
                    "Locomotive BASIC 1.1",
                    "7630682a0fd8deaa4514954c1a256224d3ae4cb6e3dd4da8e5ad8e1c5c55adc7",
                    Some(RomVariant::Language(RomLanguage::English)),
                    Some(RomSlot::Upper(0)),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("amsdos.rom")),
                info: RomInfo::from((
                    "AMSDOS 0.5",
                    "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7",
                    None,
                    Some(RomSlot::Upper(7)),
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("not found")
                )
                .count(),
            0
        );

        assert_eq!(
            harness
                .query_all_by_label("Use")
                .filter(|node| node.accesskit_node().is_disabled()
                    && node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True))
                .count(),
            3
        );
    }

    #[test]
    fn test_preferred_language_selects_the_matching_rom_variant() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_fr.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "bf87e68ff847052fecc98f84f286b23edc49d607ea8b140fc93e90259cc98ffb",
                    Some(RomVariant::Language(RomLanguage::French)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_es.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "95ee6b221aa48f2f04641948463372f6daa7ac3b56206598e5de01a49d37a071",
                    Some(RomVariant::Language(RomLanguage::Spanish)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("basic_fr.rom")),
                info: RomInfo::from((
                    "Locomotive BASIC 1.1",
                    "fc9f747896664b6c89f6fd382691cf22d0840dbf8579b35af531b55b60f54aa5",
                    Some(RomVariant::Language(RomLanguage::French)),
                    Some(RomSlot::Upper(0)),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("basic_es.rom")),
                info: RomInfo::from((
                    "Locomotive BASIC 1.1",
                    "8050045437f5127452ee51ac7cf762c748b180e27cea7884226f9d7594b7843e",
                    Some(RomVariant::Language(RomLanguage::Spanish)),
                    Some(RomSlot::Upper(0)),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("amsdos.rom")),
                info: RomInfo::from((
                    "AMSDOS 0.5",
                    "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7",
                    None,
                    Some(RomSlot::Upper(7)),
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            preferred_language: RomLanguage::Spanish,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("not found")
                )
                .is_none(),
            "all required ROMs should be listed"
        );

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("Spanish")
                )
                .count(),
            2
        );

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("French")
                )
                .count(),
            0
        );
    }

    #[test]
    fn test_preferred_language_falls_back_to_english() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_fr.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "bf87e68ff847052fecc98f84f286b23edc49d607ea8b140fc93e90259cc98ffb",
                    Some(RomVariant::Language(RomLanguage::French)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_da.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "c9b740d79546a988e12b86b16beb3dbd164b740cac64fe61afa2fcc8e591009c",
                    Some(RomVariant::Language(RomLanguage::Danish)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("basic_fr.rom")),
                info: RomInfo::from((
                    "Locomotive BASIC 1.1",
                    "fc9f747896664b6c89f6fd382691cf22d0840dbf8579b35af531b55b60f54aa5",
                    Some(RomVariant::Language(RomLanguage::French)),
                    Some(RomSlot::Upper(0)),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("basic_en.rom")),
                info: RomInfo::from((
                    "Locomotive BASIC 1.1",
                    "7630682a0fd8deaa4514954c1a256224d3ae4cb6e3dd4da8e5ad8e1c5c55adc7",
                    Some(RomVariant::Language(RomLanguage::English)),
                    Some(RomSlot::Upper(0)),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("amsdos.rom")),
                info: RomInfo::from((
                    "AMSDOS 0.5",
                    "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7",
                    None,
                    Some(RomSlot::Upper(7)),
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            preferred_language: RomLanguage::Danish,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("CPC 6128 OS (Danish)")
                )
                .is_some(),
            "OS ROM in the preferred language should be listed"
        );

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("Locomotive BASIC 1.1 (English)")
                )
                .is_some(),
            "BASIC ROM should fall back to English"
        );

        assert_eq!(
            harness
                .query_all(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("French")
                )
                .count(),
            0
        );
    }

    #[test]
    fn test_changing_the_model_reassigns_auto_configured_roms() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        open_hardware_tab(&mut harness);

        harness.get_by_label("Amstrad CPC 464").click();
        harness.run();

        open_rom_tab(&mut harness);

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("CPC 464 OS")
                )
                .is_some(),
            "CPC 464 OS should be assigned after changing the model"
        );
    }

    #[test]
    fn test_changing_the_preferred_language_reassigns_auto_configured_roms() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_da.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "c9b740d79546a988e12b86b16beb3dbd164b740cac64fe61afa2fcc8e591009c",
                    Some(RomVariant::Language(RomLanguage::Danish)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("basic_fr.rom")),
                info: RomInfo::from((
                    "Locomotive BASIC 1.1",
                    "fc9f747896664b6c89f6fd382691cf22d0840dbf8579b35af531b55b60f54aa5",
                    Some(RomVariant::Language(RomLanguage::French)),
                    Some(RomSlot::Upper(0)),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("amsdos.rom")),
                info: RomInfo::from((
                    "AMSDOS 0.5",
                    "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7",
                    None,
                    Some(RomSlot::Upper(7)),
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness.get_by_value("English").click();
        harness.run();

        harness.get_by_label("Danish").click();
        harness.run();

        harness.get_by_value("Danish").click();
        harness.run();

        harness.get_by_label("French").click();
        harness.run();

        drop(harness);

        assert_eq!(modal.access_config().assigned_roms.len(), 2);
        assert_eq!(
            modal.access_config().assigned_roms[0],
            AssignedRom {
                key: RomKey(PathBuf::from("basic_fr.rom")),
                slot: RomSlot::Upper(0)
            }
        );
        assert_eq!(
            modal.access_config().assigned_roms[1],
            AssignedRom {
                key: RomKey(PathBuf::from("amsdos.rom")),
                slot: RomSlot::Upper(7)
            }
        );
    }

    #[test]
    fn test_manual_assignment_uses_the_slot_declared_by_the_rom() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("os6128_fr.rom")),
            info: RomInfo::from((
                "CPC 6128 OS",
                "bf87e68ff847052fecc98f84f286b23edc49d607ea8b140fc93e90259cc98ffb",
                Some(RomVariant::Language(RomLanguage::French)),
                Some(RomSlot::Lower),
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness
            .get_by_label("Automatically configure based on selected hardware model")
            .click();
        harness.run();

        harness.get_by_label("Other detected original ROMs").click();
        harness.run();

        harness.get_by_label("Use").click();
        harness.run();

        drop(harness);
        assert_eq!(modal.access_config().assigned_roms.len(), 1);
        assert!(modal.access_config().assigned_roms.contains(&AssignedRom {
            key: RomKey(PathBuf::from("os6128_fr.rom")),
            slot: RomSlot::Lower
        }));
    }

    #[test]
    fn test_unchecking_use_unassigns_the_rom() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("os6128_en.rom")),
            info: RomInfo::from((
                "CPC 6128 OS",
                "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness
            .get_by_label("Automatically configure based on selected hardware model")
            .click();
        harness.run();

        assert_eq!(
            harness
                .query_all_by_label("Use")
                .filter(
                    |node| node.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True)
                )
                .count(),
            1
        );

        harness.get_by_label("Use").click();
        harness.run();

        drop(harness);
        assert!(
            modal.access_config().assigned_roms.is_empty(),
            "ROM should be unassigned after unchecking 'Use'"
        );
    }

    #[test]
    fn test_custom_roms_section_is_hidden_when_none_are_available() {
        let mut modal = SystemConfigModal {
            show: true,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("Custom ROMs")
                )
                .is_none(),
            "Custom ROMs section should be hidden when no custom ROMs are available"
        );
    }

    #[test]
    fn test_custom_rom_without_a_known_slot_prompts_for_a_slot() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("custom.rom")),
            info: RomInfo::from((
                "My Custom ROM",
                // "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                Some(RomVariant::Language(RomLanguage::English)),
                None,
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("Custom ROMs")
                )
                .is_some(),
            "Custom ROMs section should be visible"
        );

        harness.get_by_label("Use").click();
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("Select a slot for the ROM")
                )
                .is_some(),
            "assignment should prompt for a slot"
        );
    }

    #[test]
    fn test_custom_rom_can_be_assigned_to_a_numbered_upper_slot() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("custom.rom")),
            info: RomInfo::from((
                "My Custom ROM",
                "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                Some(RomVariant::Language(RomLanguage::English)),
                None,
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness.get_by_label("Use").click();
        harness.run();

        harness.get_by_label("Upper ROM").click();
        harness.run();

        let reassign_modal = harness
            .get_by_label("Upper ROM")
            .parent()
            .expect("should have a parent radio button")
            .parent()
            .expect("should have a parent modal");

        reassign_modal
            .get_by_role(egui::accesskit::Role::SpinButton)
            .click();
        harness.run();

        harness
            .get_by_role(egui::accesskit::Role::SpinButton)
            .type_text("42");
        harness.run();

        harness.key_press(egui::Key::Enter);
        harness.run();

        let reassign_modal = harness
            .get_by_label("Upper ROM")
            .parent()
            .expect("should have a parent radio button")
            .parent()
            .expect("should have a parent modal");

        reassign_modal.get_by_label("Ok").click();
        harness.run();

        drop(harness);
        assert_eq!(
            modal.access_config().assigned_roms,
            vec![AssignedRom {
                key: RomKey(PathBuf::from("custom.rom")),
                slot: RomSlot::Upper(42)
            }]
        );
    }

    #[test]
    fn test_assigning_to_an_occupied_slot_asks_before_reassigning() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_en.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                    Some(RomVariant::Language(RomLanguage::English)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("custom.rom")),
                info: RomInfo::from((
                    "My Custom ROM",
                    "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                    Some(RomVariant::Language(RomLanguage::English)),
                    None,
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let grid_row = harness
            .get_by_label("My Custom ROM (English)")
            .parent()
            .expect("should have a parent vertical group")
            .parent()
            .expect("should have a parent grid row");

        grid_row.get_by_label("Use").click();
        harness.run();

        harness.get_by_label("Lower ROM").click();
        harness.run();

        assert!(
            harness
                .query(
                    kittest::by()
                        .role(egui::accesskit::Role::Label)
                        .label_contains("occupied")
                )
                .is_some(),
            "assignment should warn the slot is occupied"
        );

        drop(harness);
        assert_eq!(
            modal.access_config().assigned_roms,
            vec![AssignedRom {
                key: RomKey(PathBuf::from("os6128_en.rom")),
                slot: RomSlot::Lower
            }]
        );
    }

    #[test]
    fn test_cancelling_a_reassignment_keeps_the_previous_rom() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_en.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                    Some(RomVariant::Language(RomLanguage::English)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("custom.rom")),
                info: RomInfo::from((
                    "My Custom ROM",
                    "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                    Some(RomVariant::Language(RomLanguage::English)),
                    None,
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let grid_row = harness
            .get_by_label("My Custom ROM (English)")
            .parent()
            .expect("should have a parent vertical group")
            .parent()
            .expect("should have a parent grid row");

        grid_row.get_by_label("Use").click();
        harness.run();

        harness.get_by_label("Lower ROM").click();
        harness.run();

        let reassign_modal = harness
            .get_by_label("Select a slot for the ROM:")
            .parent()
            .expect("should have a parent modal");

        reassign_modal.get_by_label("Cancel").click();
        harness.run();

        drop(harness);
        assert_eq!(
            modal.access_config().assigned_roms,
            vec![AssignedRom {
                key: RomKey(PathBuf::from("os6128_en.rom")),
                slot: RomSlot::Lower
            }]
        );
    }

    #[test]
    fn test_reassigning_an_auto_configured_slot_turns_auto_config_off() {
        let available_roms = vec![
            AvailableRom {
                key: RomKey(PathBuf::from("os6128_en.rom")),
                info: RomInfo::from((
                    "CPC 6128 OS",
                    "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                    Some(RomVariant::Language(RomLanguage::English)),
                    Some(RomSlot::Lower),
                )),
            },
            AvailableRom {
                key: RomKey(PathBuf::from("custom.rom")),
                info: RomInfo::from((
                    "My Custom ROM",
                    "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                    Some(RomVariant::Language(RomLanguage::English)),
                    None,
                )),
            },
        ];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        let grid_row = harness
            .get_by_label("My Custom ROM (English)")
            .parent()
            .expect("should have a parent vertical group")
            .parent()
            .expect("should have a parent grid row");

        grid_row.get_by_label("Use").click();
        harness.run();

        harness.get_by_label("Lower ROM").click();
        harness.run();

        let reassign_modal = harness
            .get_by_label("Select a slot for the ROM:")
            .parent()
            .expect("should have a parent modal");

        reassign_modal.get_by_label("Ok").click();
        harness.run();

        drop(harness);
        assert!(
            !modal.access_config().auto_config,
            "auto config should be turned off after a manual assignment"
        );
        assert_eq!(
            modal.access_config().assigned_roms,
            vec![AssignedRom {
                key: RomKey(PathBuf::from("custom.rom")),
                slot: RomSlot::Lower
            }]
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_roms_that_disappear_from_the_folder_are_unassigned() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("os6128_en.rom")),
            info: RomInfo::from((
                "CPC 6128 OS",
                "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let rom_folder = std::env::temp_dir()
            .join("ronald_test_roms_that_disappear_from_the_folder_are_unassigned");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: Some(rom_folder.clone()),
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        drop(harness);
        assert!(
            modal.access_config().assigned_roms.is_empty(),
            "ROM should be unassigned after it disappears from the folder"
        );

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    fn test_ok_persists_rom_assignments() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("os6128_en.rom")),
            info: RomInfo::from((
                "CPC 6128 OS",
                "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness.get_by_label("Ok").click();
        harness.run();

        drop(harness);
        assert_eq!(config.assigned_roms.len(), 1);
        assert_eq!(
            config.assigned_roms[0],
            AssignedRom {
                key: RomKey(PathBuf::from("os6128_en.rom")),
                slot: RomSlot::Lower
            }
        );
    }

    #[test]
    fn test_cancel_discards_rom_assignments() {
        let available_roms = vec![AvailableRom {
            key: RomKey(PathBuf::from("os6128_en.rom")),
            info: RomInfo::from((
                "CPC 6128 OS",
                "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
        }];
        let mut modal = SystemConfigModal {
            show: true,
            available_roms,
            ..Default::default()
        };

        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            assigned_roms: vec![AssignedRom {
                key: RomKey(PathBuf::from("os6128_fr.rom")),
                slot: RomSlot::Lower,
            }],
            ..Default::default()
        };

        let app = |ui: &mut egui::Ui| {
            modal.ui(ui, &mut config);
        };

        let mut harness = Harness::new_ui(app);
        harness.run();

        harness.get_by_label("Cancel").click();
        harness.run();

        drop(harness);
        assert_eq!(config.assigned_roms.len(), 1);
        assert_eq!(
            config.assigned_roms[0],
            AssignedRom {
                key: RomKey(PathBuf::from("os6128_fr.rom")),
                slot: RomSlot::Lower
            }
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_build_core_config_validates_assigned_roms_not_empty() {
        let config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: None,
            assigned_roms: Vec::new(),
            ..Default::default()
        };

        let core_config = shared(SystemConfigState::Unknown);

        build_core_config(&config, core_config.clone());

        core_config.with_mut(|state| {
            assert!(
                matches!(state, SystemConfigState::Invalid),
                "empty assigned ROMs should result in an Invalid state"
            );
        });
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_build_core_config_detects_missing_roms() {
        let rom_folder =
            std::env::temp_dir().join("ronald_test_build_core_config_detects_missing_roms");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        let config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: Some(rom_folder.clone()),
            assigned_roms: vec![AssignedRom {
                key: RomKey(rom_folder.join("os6128_en.rom")),
                slot: RomSlot::Lower,
            }],
            ..Default::default()
        };

        let core_config = shared(SystemConfigState::Unknown);

        build_core_config(&config, core_config.clone());

        core_config.with_mut(|state| {
            assert!(
                matches!(state, SystemConfigState::Invalid),
                "missing ROMs should result in an Invalid state"
            );
        });

        let _ = std::fs::remove_dir_all(&rom_folder);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_build_core_config_succeeds_with_existing_roms() {
        let rom_folder =
            std::env::temp_dir().join("ronald_test_build_core_config_succeeds_with_existing_roms");
        let _ = std::fs::remove_dir_all(&rom_folder);
        let _ = std::fs::create_dir_all(&rom_folder);

        std::fs::write(rom_folder.join("os6128_en.rom"), b"dummy content").unwrap();

        let config = SystemConfig {
            model: CpcModel::Cpc6128,
            rom_folder: Some(rom_folder.clone()),
            assigned_roms: vec![AssignedRom {
                key: RomKey(rom_folder.join("os6128_en.rom")),
                slot: RomSlot::Lower,
            }],
            ..Default::default()
        };

        let core_config = shared(SystemConfigState::Unknown);

        build_core_config(&config, core_config.clone());

        core_config.with_mut(|state| {
            assert!(
                matches!(state, SystemConfigState::Valid(_)),
                "existing ROMs should result in a Valid state"
            );
        });

        let _ = std::fs::remove_dir_all(&rom_folder);
    }
}
