use eframe::egui;

pub use ronald_core::system::{SystemConfig, CpcModel, CrtcType, DiskDrives};

#[derive(Default)]
pub struct SystemConfigModal {
    pub show: bool,
    changed_config: Option<SystemConfig>,
}

impl SystemConfigModal {
    pub fn ui(&mut self, ctx: &egui::Context, config: &mut SystemConfig) {
        if !self.show {
            return;
        }

        // Initialize temp config if not already set
        if self.changed_config.is_none() {
            self.changed_config = Some(config.clone());
        }

        egui::Modal::new("system_config_modal".into()).show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.set_max_width(400.0);
                ui.add_space(10.0);
                ui.heading("System Configuration");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                            ui.label("Model:");
                        });
                        ui.group(|ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                if let Some(config) = &mut self.changed_config {
                                    ui.radio_value(
                                        &mut config.model,
                                        CpcModel::Cpc464,
                                        "Amstrad CPC 464",
                                    );
                                    ui.radio_value(
                                        &mut config.model,
                                        CpcModel::Cpc664,
                                        "Amstrad CPC 664",
                                    );
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

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        if let Some(changed) = self.changed_config.take() {
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
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};

    #[test]
    fn test_system_config_modal_opens_and_closes() {
        let mut modal = SystemConfigModal {
            show: true,
            changed_config: None,
        };
        let mut config = SystemConfig::default();

        let app = move |ctx: &egui::Context| {
            modal.ui(ctx, &mut config);
        };

        let mut harness = Harness::new(app);
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
            changed_config: None,
        };
        let mut config = SystemConfig {
            model: CpcModel::Cpc464,
            crtc: CrtcType::Type0,
            disk_drives: DiskDrives::None,
        };

        let app = move |ctx: &egui::Context| {
            modal.ui(ctx, &mut config);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Initially None should be selected for CPC 464
        let none_option = harness.get_by_label("None");
        assert_eq!(
            none_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click on CPC 664 radio button
        harness.get_by_label("Amstrad CPC 664").click();
        harness.run();

        // Verify that "Drive A only" is now selected
        let drive_a_option = harness.get_by_label("Drive A only");
        assert_eq!(
            drive_a_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn test_restore_defaults_button() {
        let mut modal = SystemConfigModal {
            show: true,
            changed_config: None,
        };
        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            crtc: CrtcType::Type4,
            disk_drives: DiskDrives::Two,
        };

        let app = move |ctx: &egui::Context| {
            modal.ui(ctx, &mut config);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Verify initial state - CPC 6128 should be selected
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click the Restore Defaults button
        harness.get_by_label("Restore Defaults").click();
        harness.run();

        // Verify UI shows defaults - CPC 464 should now be selected
        let cpc464_option = harness.get_by_label("Amstrad CPC 464");
        assert_eq!(
            cpc464_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let type0_option = harness.get_by_label("Type 0 (HD6845S/UM6845)");
        assert_eq!(
            type0_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let none_option = harness.get_by_label("None");
        assert_eq!(
            none_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn test_crtc_selection_changes_config() {
        let mut modal = SystemConfigModal {
            show: true,
            changed_config: None,
        };
        let mut config = SystemConfig::default();

        let app = move |ctx: &egui::Context| {
            modal.ui(ctx, &mut config);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Initially Type 0 should be selected
        let type0_option = harness.get_by_label("Type 0 (HD6845S/UM6845)");
        assert_eq!(
            type0_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        // Click on Type 2 CRTC option
        harness.get_by_label("Type 2 (MC6845)").click();
        harness.run();

        // Verify Type 2 is now selected in UI
        let type2_option = harness.get_by_label("Type 2 (MC6845)");
        assert_eq!(
            type2_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn test_cancel_button_preserves_config() {
        let mut modal = SystemConfigModal {
            show: true,
            changed_config: None,
        };
        let mut config = SystemConfig {
            model: CpcModel::Cpc6128,
            crtc: CrtcType::Type4,
            disk_drives: DiskDrives::Two,
        };

        let app = |ctx: &egui::Context| {
            modal.ui(ctx, &mut config);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Verify initial UI state - CPC 6128 should be selected
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.node().toggled(),
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
        let app = |ctx: &egui::Context| {
            modal.ui(ctx, &mut config);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Verify original values are still selected in UI
        let cpc6128_option = harness.get_by_label("Amstrad CPC 6128");
        assert_eq!(
            cpc6128_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let type4_option = harness.get_by_label("Type 4 (AMS40226)");
        assert_eq!(
            type4_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );

        let drives_ab_option = harness.get_by_label("Drives A and B");
        assert_eq!(
            drives_ab_option.node().toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }
}
