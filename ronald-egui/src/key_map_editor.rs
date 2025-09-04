use std::{collections::HashMap, fmt::Display};

use eframe::egui;
use ronald_core::constants::{KEYS, KeyDefinition};

use crate::key_mapper::{KeyMapStore, KeyMapper};

mod layout;
use layout::KEY_LAYOUTS;

const SIZE: usize = 100; // TODO: use this in KeyLayout definitions
const PADDING: usize = 8;
const CORNER_RADIUS: usize = 10;
const STROKE_WIDTH: usize = 3;

struct KeyLayout {
    name: &'static str,
    x: usize,
    y: usize,
    width: usize,
    label: &'static str,
}

impl KeyLayout {
    fn rect(&self, image_rect: egui::Rect, scale_x: f32, scale_y: f32) -> egui::Rect {
        let key_x = image_rect.left() + (self.x as f32 + PADDING as f32) * scale_x;
        let key_y = image_rect.top() + (self.y as f32 + PADDING as f32) * scale_y;

        let (key_width, key_height) = if self.name == "Enter" {
            // Enter key: full height, upper part width
            (
                (self.width as f32 - 2.0 * PADDING as f32) * scale_x,
                (2.0 * SIZE as f32 - 2.0 * PADDING as f32) * scale_y,
            )
        } else {
            // Regular keys
            (
                (self.width as f32 - 2.0 * PADDING as f32) * scale_x,
                (SIZE as f32 - 2.0 * PADDING as f32) * scale_y,
            )
        };

        egui::Rect::from_min_size(egui::pos2(key_x, key_y), egui::vec2(key_width, key_height))
    }

    fn contains(&self, pos: egui::Pos2) -> bool {
        let x = (self.x + PADDING) as f32;
        let y = (self.y + PADDING) as f32;
        let width = (self.width - 2 * PADDING) as f32;
        let height = if self.name == "Enter" {
            (2 * SIZE - 2 * PADDING) as f32
        } else {
            (SIZE - 2 * PADDING) as f32
        };

        if pos.x < x || pos.x > x + width || pos.y < y || pos.y > y + height {
            return false;
        }

        if self.name == "Enter" {
            let wide_section_height = (SIZE - 2 * PADDING) as f32;
            if pos.y >= y && pos.y <= y + wide_section_height && pos.x >= x && pos.x <= x + width {
                return true;
            }

            let narrow_section_x = (self.x + PADDING + SIZE / 4) as f32;
            let narrow_section_width = (self.width - 2 * PADDING - SIZE / 4) as f32;
            if pos.x < narrow_section_x || pos.x > narrow_section_x + narrow_section_width {
                return false;
            }
        }

        true
    }
}

impl Display for KeyLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name == "Enter" {
            // draw return key
            write!(
                f,
                r#"<path d="M {} {} h {}
                q {CORNER_RADIUS} 0 {CORNER_RADIUS} {CORNER_RADIUS}
                v {}
                q 0 {CORNER_RADIUS} -{CORNER_RADIUS} {CORNER_RADIUS}
                h -{}
                q -{CORNER_RADIUS} 0 -{CORNER_RADIUS} -{CORNER_RADIUS}
                v -{}
                q 0 -{CORNER_RADIUS} -{CORNER_RADIUS} -{CORNER_RADIUS}
                h -{}
                q -{CORNER_RADIUS} 0 -{CORNER_RADIUS} -{CORNER_RADIUS}
                v -{}
                q 0 -{CORNER_RADIUS} {CORNER_RADIUS} -{CORNER_RADIUS}" stroke-width="{STROKE_WIDTH}"/>"#,
                self.x + PADDING + CORNER_RADIUS,
                self.y + PADDING,
                self.width - 2 * PADDING - 2 * CORNER_RADIUS,
                2 * SIZE - 2 * PADDING - 2 * CORNER_RADIUS,
                15 * SIZE / 10 - 2 * PADDING - 2 * CORNER_RADIUS,
                SIZE - 2 * CORNER_RADIUS,
                SIZE / 4 - 2 * CORNER_RADIUS,
                SIZE - 2 * PADDING - 2 * CORNER_RADIUS,
            )
        } else {
            write!(
                f,
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" stroke-width="{STROKE_WIDTH}"/>"#,
                self.x + PADDING,
                self.y + PADDING,
                self.width - 2 * PADDING,
                SIZE - 2 * PADDING,
                CORNER_RADIUS,
            )
        }
    }
}

pub struct KeyMapEditor {
    pub show: bool,
    hovered_key: Option<&'static str>,
    key_definitions: HashMap<&'static str, KeyDefinition>,
    listening: Option<(&'static str, bool)>,
}

impl KeyMapEditor {
    pub fn ui<K>(&mut self, ctx: &egui::Context, key_mapper: &mut KeyMapper<K>)
    where
        K: KeyMapStore,
    {
        if !self.show {
            return;
        }

        egui::Modal::new("key_bindings_modal".into()).show(ctx, |ui| {
            self.render_binding_listener_modal(ctx, key_mapper);

            ui.add_space(10.0);
            ui.heading("Key Bindings");
            ui.add_space(20.0);

            ui.label("Click keys to set bindings. Shift-click to set bindings for shifted keys. The guest system's Shift keys themselves cannot be bound.");
            ui.add_space(20.0);

            let svg = self.generate_keyboard_svg(ui);
            let image_response = self.render_keyboard_image(ui, svg);

            self.handle_key_interactions(ctx, ui, image_response);

            ui.add_space(15.0);
            if ui.button("Close").clicked() {
                self.show = false;
            }
        });
    }

    fn render_binding_listener_modal<K>(
        &mut self,
        ctx: &egui::Context,
        key_mapper: &mut KeyMapper<K>,
    ) where
        K: KeyMapStore,
    {
        if let Some((hovered_key, shifted)) = self.listening {
            egui::Modal::new("key_binding_listener".into()).show(ctx, |ui| {
                ui.set_max_width(350.0);
                if shifted {
                    ui.label(format!(
                        "Press a key to bind to \"Shift + {hovered_key}\" on the guest system."
                    ));
                } else {
                    ui.label(format!(
                        "Press a key to bind to \"{hovered_key}\" on the guest system."
                    ));
                }

                match key_mapper.binding(hovered_key, shifted) {
                    Some(host_key) => {
                        ui.label(format!("Currently bound to {host_key}"));
                    }
                    None => {
                        ui.label("No binding set yet.");
                    }
                }

                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.listening = None;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(host_key) = key_mapper.binding(hovered_key, shifted) {
                            if ui.button("Clear Binding").clicked() {
                                let _ = key_mapper.clear_binding(hovered_key, shifted);
                            }
                        }
                    });
                });

                ui.input(|input| {
                    if let Ok(true) = key_mapper.try_set_binding(hovered_key, shifted, input) {
                        self.listening = None;
                    }
                });
            });
        }
    }

    fn generate_keyboard_svg(&self, ui: &mut egui::Ui) -> String {
        let style = ui.style();
        let stroke_color = style.visuals.text_color().to_hex();
        let fill_color = style.visuals.text_color().to_hex();
        let hover_stroke_color = style.visuals.strong_text_color().to_hex();
        let hover_fill_color = style.visuals.selection.bg_fill.to_hex();
        let hover_text_color = style.visuals.selection.stroke.color.to_hex();

        let mut svg = String::new();
        svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2200 500">"#);
        for key in KEY_LAYOUTS {
            if key.name == "Shift" {
                continue;
            }

            match self.hovered_key {
                Some(hovered_key) if hovered_key == key.name => {
                    svg.push_str(&format!(
                        r#"<g stroke="{}" fill="{}">"#,
                        hover_stroke_color, hover_fill_color
                    ));
                    svg.push_str(key.to_string().as_str());
                    svg.push_str(r#"</g>"#);
                    svg.push_str(&format!(
                        r#"<g stroke="{}" fill="{}">"#,
                        hover_text_color, hover_text_color
                    ));
                    svg.push_str(key.label);
                    svg.push_str(r#"</g>"#);
                }
                _ => {
                    svg.push_str(&format!(
                        r#"<g stroke="{}" fill="transparent">"#,
                        stroke_color
                    ));
                    svg.push_str(key.to_string().as_str());
                    svg.push_str(r#"</g>"#);
                    svg.push_str(&format!(
                        r#"<g stroke="{}" fill="{}">"#,
                        fill_color, fill_color
                    ));
                    svg.push_str(key.label);
                    svg.push_str(r#"</g>"#);
                }
            }
        }
        svg.push_str(&format!(
            r#"<g stroke="{}" fill="{}">"#,
            fill_color, fill_color
        ));
        svg.push_str(include_str!("../assets/keys/JoystickIcon.partial.svg"));
        svg.push_str(r#"</g>"#);
        svg.push_str(r#"</svg>"#);

        svg
    }

    fn render_keyboard_image(&self, ui: &mut egui::Ui, svg: String) -> egui::Response {
        let uri = match self.hovered_key {
            Some(key) => format!("bytes://keyboard_layout_{}.svg", key),
            None => "bytes://keyboard_layout.svg".to_string(),
        };

        ui.add(
            egui::Image::new(egui::ImageSource::Bytes {
                uri: uri.into(),
                bytes: svg.into_bytes().into(),
            })
            .fit_to_exact_size(egui::vec2(1100.0, 250.0)),
        )
    }

    fn handle_key_interactions(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        image_response: egui::Response,
    ) {
        let image_rect = image_response.rect;
        let scale_x = image_rect.width() / 2200.0; // SVG viewBox is 2200x500
        let scale_y = image_rect.height() / 500.0;

        // Clear hover state at beginning of each frame
        self.hovered_key = None;

        for key_layout in KEY_LAYOUTS {
            // Skip Shift keys as they cannot be bound
            if key_layout.name == "Shift" {
                continue;
            }

            let key_rect = key_layout.rect(image_rect, scale_x, scale_y);
            let key_id = ui.make_persistent_id(("key_map_editor", key_layout.name));
            let key_response = ui.interact(
                key_rect,
                key_id,
                egui::Sense::hover().union(egui::Sense::click()),
            );

            // Add accessibility label
            key_response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    true,
                    format!("{} key", key_layout.name),
                )
            });

            // For Enter key, use precise L-shaped hit detection
            let is_valid_hit = if key_layout.name == "Enter" {
                if let Some(cursor_pos) = ctx.pointer_interact_pos() {
                    // Convert cursor position back to SVG coordinates
                    let svg_x = (cursor_pos.x - image_rect.left()) / scale_x;
                    let svg_y = (cursor_pos.y - image_rect.top()) / scale_y;
                    let svg_pos = egui::Pos2::new(svg_x, svg_y);
                    key_layout.contains(svg_pos)
                } else {
                    false
                }
            } else {
                true
            };

            self.handle_key_activation(key_layout, ui, &key_response, is_valid_hit);

            // Handle hover and focus state
            if (key_response.hovered() && is_valid_hit) || key_response.has_focus() {
                self.hovered_key = Some(key_layout.name);
            }
        }
    }

    fn handle_key_activation(
        &mut self,
        key_layout: &KeyLayout,
        ui: &mut egui::Ui,
        key_response: &egui::Response,
        is_valid_hit: bool,
    ) {
        let is_mouse_activated = key_response.clicked() && is_valid_hit;
        let is_keyboard_activated = key_response.has_focus()
            && ui.input(|input| {
                input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
            });

        if self.listening.is_none() && (is_mouse_activated || is_keyboard_activated) {
            log::debug!("Activated key: {}", key_layout.name);
            let shiftable = self
                .key_definitions
                .get(key_layout.name)
                .expect("Key not found in KEYS")
                .shiftable;
            let shift_held = ui.input(|input| input.modifiers.contains(egui::Modifiers::SHIFT));

            if shiftable && shift_held {
                self.listening = Some((key_layout.name, true));
            } else if !shift_held {
                self.listening = Some((key_layout.name, false));
            }
        }
    }
}

impl Default for KeyMapEditor {
    fn default() -> Self {
        Self {
            show: false,
            hovered_key: None,
            listening: None,
            key_definitions: HashMap::from(KEYS),
        }
    }
}

#[cfg(test)]
mod gui_tests {
    use super::*;
    use crate::key_mapper::{KeyMap, KeyMapStore, KeyMapper};
    use egui::accesskit::Role;
    use egui_kittest::{Harness, kittest::Queryable};

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

    fn create_input_state_with_key(
        key: egui::Key,
        modifiers: egui::Modifiers,
        pressed: bool,
    ) -> egui::InputState {
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
    fn test_key_map_editor_modal_closed_by_default() {
        let mut key_map_editor = KeyMapEditor::default();
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        // Initially modal should not be shown
        key_map_editor.show = false;

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);

        // Modal should not be visible initially
        assert!(
            harness
                .query_by_label("Click keys to set bindings")
                .is_none()
        );
    }

    #[test]
    fn test_key_map_editor_modal_can_be_closed() {
        let mut key_map_editor = KeyMapEditor::default();
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        // Modal should be shown
        key_map_editor.show = true;

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Modal should be visible
        harness.get_by_label("Click keys to set bindings. Shift-click to set bindings for shifted keys. The guest system's Shift keys themselves cannot be bound.");

        // Close the modal using the Close button
        harness.get_by_label("Close").click();
        harness.run();

        // Modal should be hidden again
        assert!(
            harness
                .query_by_label("Click keys to set bindings")
                .is_none()
        );
    }

    #[test]
    fn test_key_binding_dialog_opens_on_click() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Get the A key by its accessibility label
        let a_key = harness.get_by_label("A key");

        // Click on the A key
        a_key.click();
        harness.run();

        // Verify binding dialog opened for A key
        harness.get_by_label("Press a key to bind to \"A\" on the guest system.");
        harness.get_by_label("No binding set yet.");
        harness.get_by_label("Cancel");
    }

    #[test]
    fn test_shifted_key_binding_dialog() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Get the A key by its accessibility label
        let a_key = harness.get_by_label("A key");

        // Press shift and click on the A key
        a_key.key_down(egui_kittest::kittest::Key::Shift);
        a_key.click();
        harness.run();

        // Verify shifted binding dialog opened
        harness.get_by_label("Press a key to bind to \"Shift + A\" on the guest system.");
        harness.get_by_label("No binding set yet.");
        harness.get_by_label("Cancel");
    }

    #[test]
    fn test_key_binding_dialog_cancellation() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Open binding dialog
        harness.get_by_label("A key").click();
        harness.run();

        // Verify dialog is open
        harness.get_by_label("Press a key to bind to \"A\" on the guest system.");

        // Click Cancel button
        harness.get_by_label("Cancel").click();
        harness.run();

        // Verify dialog text is no longer present
        assert!(
            harness
                .query_by_label("Press a key to bind to \"A\" on the guest system.")
                .is_none()
        );

        // Verify no binding was set by reopening the listener modal
        harness.get_by_label("A key").click();
        harness.run();

        // Should still show "No binding set yet."
        harness.get_by_label("No binding set yet.");
    }

    #[test]
    fn test_clear_existing_binding() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        // Pre-set a binding for A key to Q
        let input_state = create_input_state_with_key(egui::Key::Q, egui::Modifiers::NONE, true);
        key_mapper
            .try_set_binding("A", false, &input_state)
            .unwrap();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Get the A key by its accessibility label
        let a_key = harness.get_by_label("A key");

        // Open binding dialog
        a_key.click();
        harness.run();

        // Verify existing binding is shown
        harness.get_by_label("Currently bound to Q");

        // Click Clear Binding button
        harness.get_by_label("Clear Binding").click();
        harness.run();

        // Verify dialog shows no binding
        harness.get_by_label("No binding set yet.");
        assert!(harness.query_by_label("Currently bound to Q").is_none());
    }

    #[test]
    fn test_shift_key_exclusion() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Verify the instruction text mentions that Shift keys cannot be bound
        harness.get_by_label("Click keys to set bindings. Shift-click to set bindings for shifted keys. The guest system's Shift keys themselves cannot be bound.");

        // Verify that Shift keys don't have accessibility labels (they should not be clickable)
        // This proves they are excluded from interaction
        assert!(harness.query_by_label("Shift key").is_none());

        // Verify we can still find other keys like A
        harness.get_by_label("A key");
    }

    #[test]
    fn test_keyboard_navigation() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Tab once to highlight the first key (should be Escape key)
        harness
            .get_by_role(Role::Image)
            .key_down(egui_kittest::kittest::Key::Tab);
        harness.run();

        // Press Enter to activate the focused key
        harness
            .get_by_role(Role::Image)
            .key_down(egui_kittest::kittest::Key::Enter);
        harness.run();

        // Verify binding dialog opened for Escape key
        harness.get_by_label("Press a key to bind to \"Escape\" on the guest system.");
        harness.get_by_label("No binding set yet.");
        harness.get_by_label("Cancel");
    }
}
