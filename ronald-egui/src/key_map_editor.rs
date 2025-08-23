use std::{collections::HashMap, fmt::Display};

use eframe::egui;
use ronald_core::constants::{KEYS, KeyDefinition};

use crate::key_mapper::{KeyMapStore, KeyMapper};

const SIZE: usize = 100; // TODO: use this in GuestKey definitions
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
    fn contains_pos(&self, pos: egui::Pos2) -> bool {
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
    key_layouts: Vec<KeyLayout>,
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

        let modal = egui::Modal::new("key_bindings_modal".into()).show(ctx, |ui| {
            ui.label("Click keys to set bindings. Shift-click to set bindings for shifted keys. The guest system's Shift keys themselves cannot be bound.");

            // Get theme-appropriate colors
            let style = ui.style();
            let stroke_color = style.visuals.text_color().to_hex();
            let fill_color = style.visuals.text_color().to_hex();
            let hover_stroke_color = style.visuals.strong_text_color().to_hex();
            let hover_fill_color = style.visuals.selection.bg_fill.to_hex();
            let hover_text_color = style.visuals.selection.stroke.color.to_hex();

            let mut svg = String::new();
            svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2200 500">"#);
            for key in &self.key_layouts {
                if key.name == "Shift" {
                    continue;
                }

                match self.hovered_key {
                    Some(hovered_key) if hovered_key == key.name => {
                        svg.push_str(&format!(r#"<g stroke="{}" fill="{}">"#, hover_stroke_color, hover_fill_color));
                        svg.push_str(key.to_string().as_str());
                        svg.push_str(r#"</g>"#);
                        svg.push_str(&format!(r#"<g stroke="{}" fill="{}">"#, hover_text_color, hover_text_color));
                        svg.push_str(key.label);
                        svg.push_str(r#"</g>"#);
                    }
                    _ => {
                        svg.push_str(&format!(r#"<g stroke="{}" fill="transparent">"#, stroke_color));
                        svg.push_str(key.to_string().as_str());
                        svg.push_str(r#"</g>"#);
                        svg.push_str(&format!(r#"<g stroke="{}" fill="{}">"#, fill_color, fill_color));
                        svg.push_str(key.label);
                        svg.push_str(r#"</g>"#);
                    }
                }
            }
            svg.push_str(&format!(r#"<g stroke="{}" fill="{}">"#, fill_color, fill_color));
            svg.push_str(include_str!("../assets/keys/JoystickIcon.partial.svg"));
            svg.push_str(r#"</g>"#);

            svg.push_str(r#"</svg>"#);

            // Use a unique URI based on hovered key to avoid texture conflicts
            let uri = match self.hovered_key {
                Some(key) => format!("bytes://keyboard_layout_{}.svg", key),
                None => "bytes://keyboard_layout.svg".to_string(),
            };

            let image_response = ui.add(
                egui::Image::new(egui::ImageSource::Bytes {
                    uri: uri.into(),
                    bytes: svg.into_bytes().into(),
                })
                .fit_to_exact_size(egui::vec2(1100.0, 250.0))
            );

            // Allocate individual key rectangles for accessibility and interaction
            let image_rect = image_response.rect;
            let scale_x = image_rect.width() / 2200.0; // SVG viewBox is 2200x500
            let scale_y = image_rect.height() / 500.0;

            // Clear hover state at beginning of each frame
            self.hovered_key = None;

            for key_layout in &self.key_layouts {
                // Skip Shift keys as they cannot be bound
                if key_layout.name == "Shift" {
                    continue;
                }

                // Calculate scaled position and size
                let key_x = image_rect.left() + (key_layout.x as f32 + PADDING as f32) * scale_x;
                let key_y = image_rect.top() + (key_layout.y as f32 + PADDING as f32) * scale_y;

                let (key_width, key_height) = if key_layout.name == "Enter" {
                    // Enter key: full height, upper part width
                    (
                        (key_layout.width as f32 - 2.0 * PADDING as f32) * scale_x,
                        (2.0 * SIZE as f32 - 2.0 * PADDING as f32) * scale_y,
                    )
                } else {
                    // Regular keys
                    (
                        (key_layout.width as f32 - 2.0 * PADDING as f32) * scale_x,
                        (SIZE as f32 - 2.0 * PADDING as f32) * scale_y,
                    )
                };

                let key_rect = egui::Rect::from_min_size(
                    egui::pos2(key_x, key_y),
                    egui::vec2(key_width, key_height),
                );

                let key_response = ui.allocate_rect(key_rect, egui::Sense::hover().union(egui::Sense::click()));

                // Add accessibility label
                key_response.widget_info(|| egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    true,
                    format!("{} key", key_layout.name)
                ));

                // For Enter key, use precise L-shaped hit detection
                let is_valid_hit = if key_layout.name == "Enter" {
                    if let Some(cursor_pos) = ctx.pointer_interact_pos() {
                        // Convert cursor position back to SVG coordinates
                        let svg_x = (cursor_pos.x - image_rect.left()) / scale_x;
                        let svg_y = (cursor_pos.y - image_rect.top()) / scale_y;
                        let svg_pos = egui::Pos2::new(svg_x, svg_y);
                        key_layout.contains_pos(svg_pos)
                    } else {
                        false
                    }
                } else {
                    true
                };

                // Handle click events
                if self.listening.is_none() && key_response.clicked() && is_valid_hit {
                    log::debug!("Clicked on key: {}", key_layout.name);
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

                // Handle hover state
                if key_response.hovered() && is_valid_hit {
                    self.hovered_key = Some(key_layout.name);
                }
            }

            if let Some((hovered_key, shifted)) = self.listening {
                egui::Modal::new("key_binding_listener".into()).show(ctx, |ui| {
                    if shifted {
                        ui.label(format!("Press a key to bind to \"Shift + {hovered_key}\" on the guest system."));
                    } else {
                        ui.label(format!("Press a key to bind to \"{hovered_key}\" on the guest system."));
                    }

                    match key_mapper.binding(hovered_key, shifted) {
                        Some(host_key) => {
                            ui.label(format!("Currently bound to {host_key}"));
                            if ui.button("Clear Binding").clicked() {
                                let _ = key_mapper.clear_binding(hovered_key, shifted);
                            }
                        }
                        None => {
                            ui.label("No binding set yet.");
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Cancel").clicked() {
                            self.listening = None;
                        }
                    });

                    ui.input(|input| {
                        if let Ok(true) =
                            key_mapper.try_set_binding(hovered_key, shifted, input)
                        {
                            self.listening = None;
                        }
                    });
                });
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Close").clicked() {
                    self.show = false;
                }
            });
        });
    }
}

impl Default for KeyMapEditor {
    fn default() -> Self {
        Self {
            show: false,
            hovered_key: None,
            listening: None,
            key_definitions: HashMap::from(KEYS),
            key_layouts: vec![
                KeyLayout {
                    name: "Escape",
                    x: 0,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Escape.partial.svg"),
                },
                KeyLayout {
                    name: "Key1",
                    x: 100,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key1.partial.svg"),
                },
                KeyLayout {
                    name: "Key2",
                    x: 200,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key2.partial.svg"),
                },
                KeyLayout {
                    name: "Key3",
                    x: 300,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key3.partial.svg"),
                },
                KeyLayout {
                    name: "Key4",
                    x: 400,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key4.partial.svg"),
                },
                KeyLayout {
                    name: "Key5",
                    x: 500,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key5.partial.svg"),
                },
                KeyLayout {
                    name: "Key6",
                    x: 600,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key6.partial.svg"),
                },
                KeyLayout {
                    name: "Key7",
                    x: 700,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key7.partial.svg"),
                },
                KeyLayout {
                    name: "Key8",
                    x: 800,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key8.partial.svg"),
                },
                KeyLayout {
                    name: "Key9",
                    x: 900,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key9.partial.svg"),
                },
                KeyLayout {
                    name: "Key0",
                    x: 1000,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key0.partial.svg"),
                },
                KeyLayout {
                    name: "Minus",
                    x: 1100,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Minus.partial.svg"),
                },
                KeyLayout {
                    name: "Caret",
                    x: 1200,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Caret.partial.svg"),
                },
                KeyLayout {
                    name: "Clear",
                    x: 1300,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Clear.partial.svg"),
                },
                KeyLayout {
                    name: "Delete",
                    x: 1400,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Delete.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad7",
                    x: 1500,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad7.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad8",
                    x: 1600,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad8.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad9",
                    x: 1700,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad9.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickUp",
                    x: 2000,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickUp.partial.svg"),
                },
                KeyLayout {
                    name: "Tab",
                    x: 0,
                    y: 100,
                    width: 125,
                    label: include_str!("../assets/keys/Tab.partial.svg"),
                },
                KeyLayout {
                    name: "Q",
                    x: 125,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Q.partial.svg"),
                },
                KeyLayout {
                    name: "W",
                    x: 225,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/W.partial.svg"),
                },
                KeyLayout {
                    name: "E",
                    x: 325,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/E.partial.svg"),
                },
                KeyLayout {
                    name: "R",
                    x: 425,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/R.partial.svg"),
                },
                KeyLayout {
                    name: "T",
                    x: 525,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/T.partial.svg"),
                },
                KeyLayout {
                    name: "Y",
                    x: 625,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Y.partial.svg"),
                },
                KeyLayout {
                    name: "U",
                    x: 725,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/U.partial.svg"),
                },
                KeyLayout {
                    name: "I",
                    x: 825,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/I.partial.svg"),
                },
                KeyLayout {
                    name: "O",
                    x: 925,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/O.partial.svg"),
                },
                KeyLayout {
                    name: "P",
                    x: 1025,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/P.partial.svg"),
                },
                KeyLayout {
                    name: "At",
                    x: 1125,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/At.partial.svg"),
                },
                KeyLayout {
                    name: "BracketLeft",
                    x: 1225,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/BracketLeft.partial.svg"),
                },
                KeyLayout {
                    name: "Enter",
                    x: 1325,
                    y: 100,
                    width: 175,
                    label: include_str!("../assets/keys/Enter.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad4",
                    x: 1500,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad4.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad5",
                    x: 1600,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad5.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad6",
                    x: 1700,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad6.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickLeft",
                    x: 1900,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickLeft.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickRight",
                    x: 2100,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickRight.partial.svg"),
                },
                KeyLayout {
                    name: "CapsLock",
                    x: 0,
                    y: 200,
                    width: 150,
                    label: include_str!("../assets/keys/CapsLock.partial.svg"),
                },
                KeyLayout {
                    name: "A",
                    x: 150,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/A.partial.svg"),
                },
                KeyLayout {
                    name: "S",
                    x: 250,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/S.partial.svg"),
                },
                KeyLayout {
                    name: "D",
                    x: 350,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/D.partial.svg"),
                },
                KeyLayout {
                    name: "F",
                    x: 450,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/F.partial.svg"),
                },
                KeyLayout {
                    name: "G",
                    x: 550,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/G.partial.svg"),
                },
                KeyLayout {
                    name: "H",
                    x: 650,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/H.partial.svg"),
                },
                KeyLayout {
                    name: "J",
                    x: 750,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/J.partial.svg"),
                },
                KeyLayout {
                    name: "K",
                    x: 850,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/K.partial.svg"),
                },
                KeyLayout {
                    name: "L",
                    x: 950,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/L.partial.svg"),
                },
                KeyLayout {
                    name: "Colon",
                    x: 1050,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Colon.partial.svg"),
                },
                KeyLayout {
                    name: "Semicolon",
                    x: 1150,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Semicolon.partial.svg"),
                },
                KeyLayout {
                    name: "BracketRight",
                    x: 1250,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/BracketRight.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad1",
                    x: 1500,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad1.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad2",
                    x: 1600,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad2.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad3",
                    x: 1700,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad3.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickDown",
                    x: 2000,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickDown.partial.svg"),
                },
                KeyLayout {
                    name: "Shift",
                    x: 0,
                    y: 300,
                    width: 200,
                    label: include_str!("../assets/keys/ShiftLeft.partial.svg"),
                },
                KeyLayout {
                    name: "Z",
                    x: 200,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Z.partial.svg"),
                },
                KeyLayout {
                    name: "X",
                    x: 300,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/X.partial.svg"),
                },
                KeyLayout {
                    name: "C",
                    x: 400,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/C.partial.svg"),
                },
                KeyLayout {
                    name: "V",
                    x: 500,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/V.partial.svg"),
                },
                KeyLayout {
                    name: "B",
                    x: 600,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/B.partial.svg"),
                },
                KeyLayout {
                    name: "N",
                    x: 700,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/N.partial.svg"),
                },
                KeyLayout {
                    name: "M",
                    x: 800,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/M.partial.svg"),
                },
                KeyLayout {
                    name: "Comma",
                    x: 900,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Comma.partial.svg"),
                },
                KeyLayout {
                    name: "Period",
                    x: 1000,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Period.partial.svg"),
                },
                KeyLayout {
                    name: "Slash",
                    x: 1100,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Slash.partial.svg"),
                },
                KeyLayout {
                    name: "Backslash",
                    x: 1200,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Backslash.partial.svg"),
                },
                KeyLayout {
                    name: "Shift",
                    x: 1300,
                    y: 300,
                    width: 200,
                    label: include_str!("../assets/keys/ShiftRight.partial.svg"),
                },
                KeyLayout {
                    name: "Numpad0",
                    x: 1500,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad0.partial.svg"),
                },
                KeyLayout {
                    name: "ArrowUp",
                    x: 1600,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowUp.partial.svg"),
                },
                KeyLayout {
                    name: "NumpadPeriod",
                    x: 1700,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/NumpadPeriod.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickFire1",
                    x: 1900,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickFire1.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickFire2",
                    x: 2000,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickFire2.partial.svg"),
                },
                KeyLayout {
                    name: "JoystickFire3",
                    x: 2100,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickFire3.partial.svg"),
                },
                KeyLayout {
                    name: "Control",
                    x: 0,
                    y: 400,
                    width: 200,
                    label: include_str!("../assets/keys/Control.partial.svg"),
                },
                KeyLayout {
                    name: "Copy",
                    x: 200,
                    y: 400,
                    width: 175,
                    label: include_str!("../assets/keys/Copy.partial.svg"),
                },
                KeyLayout {
                    name: "Space",
                    x: 375,
                    y: 400,
                    width: 800,
                    label: "",
                },
                KeyLayout {
                    name: "NumpadEnter",
                    x: 1175,
                    y: 400,
                    width: 325,
                    label: include_str!("../assets/keys/NumpadEnter.partial.svg"),
                },
                KeyLayout {
                    name: "ArrowLeft",
                    x: 1500,
                    y: 400,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowLeft.partial.svg"),
                },
                KeyLayout {
                    name: "ArrowDown",
                    x: 1600,
                    y: 400,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowDown.partial.svg"),
                },
                KeyLayout {
                    name: "ArrowRight",
                    x: 1700,
                    y: 400,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowRight.partial.svg"),
                },
            ],
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
    fn test_key_map_editor_modal_opens_and_closes() {
        let mut key_map_editor = KeyMapEditor::default();
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        // Initially modal should not be shown
        key_map_editor.show = false;

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);

        // Modal should not be visible initially
        assert!(harness.try_get_by("Click keys to set bindings").is_none());

        // Show the modal
        key_map_editor.show = true;
        harness.run();

        // Modal should now be visible
        harness.get_by("Click keys to set bindings");

        // Close the modal using the Close button
        harness.get_by("Close").click();
        harness.run();

        // Modal should be hidden again
        assert!(harness.try_get_by("Click keys to set bindings").is_none());
    }

    #[test]
    fn test_key_layout_hover_detection() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Find the Q key layout from the key_layouts vector
        let q_key_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "Q")
            .expect("Q key layout should exist");

        // Calculate position within the key bounds (center of the key)
        // Convert from SVG coordinates (2200x500 viewBox) to display coordinates (1100x250)
        let q_key_center_x = (q_key_layout.x + q_key_layout.width / 2) as f32 / 2.0;
        let q_key_center_y = (q_key_layout.y + SIZE / 2) as f32 / 2.0;
        let q_key_pos = egui::Pos2::new(q_key_center_x, q_key_center_y);

        // keyboard_image.hover_at(q_key_pos);
        harness.run();

        // Verify that Q key is now hovered
        assert_eq!(key_map_editor.hovered_key, Some("Q"));

        // Move hover to empty area (outside all keys)
        // keyboard_image.hover_at(egui::Pos2::new(0.0, 0.0));
        harness.run();

        // Verify hover is cleared
        assert_eq!(key_map_editor.hovered_key, None);
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

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Find the A key layout
        let a_key_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "A")
            .expect("A key layout should exist");

        // Calculate center position of A key
        let a_key_center_x = (a_key_layout.x + a_key_layout.width / 2) as f32 / 2.0;
        let a_key_center_y = (a_key_layout.y + SIZE / 2) as f32 / 2.0;
        let a_key_pos = egui::Pos2::new(a_key_center_x, a_key_center_y);

        // First hover over the key to select it
        keyboard_image.hover_at(a_key_pos);
        harness.run();

        // Verify A key is hovered
        assert_eq!(key_map_editor.hovered_key, Some("A"));

        // Click on the A key
        keyboard_image.click_at(a_key_pos);
        harness.run();

        // Verify binding dialog opened
        assert_eq!(key_map_editor.listening, Some(("A", false))); // Not shifted

        // Verify dialog text appears
        harness.get_by("Press a key to bind to \"A\" on the guest system.");
        harness.get_by("No binding set yet.");
        harness.get_by("Cancel");
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

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Find the A key layout (A is shiftable according to KEYS constant)
        let a_key_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "A")
            .expect("A key layout should exist");

        // Calculate center position of A key
        let a_key_center_x = (a_key_layout.x + a_key_layout.width / 2) as f32 / 2.0;
        let a_key_center_y = (a_key_layout.y + SIZE / 2) as f32 / 2.0;
        let a_key_pos = egui::Pos2::new(a_key_center_x, a_key_center_y);

        // First hover over the key to select it
        keyboard_image.hover_at(a_key_pos);
        harness.run();

        // Verify A key is hovered
        assert_eq!(key_map_editor.hovered_key, Some("A"));

        // Shift+click on the A key
        keyboard_image.click_at_with_modifiers(a_key_pos, egui::Modifiers::SHIFT);
        harness.run();

        // Verify shifted binding dialog opened
        assert_eq!(key_map_editor.listening, Some(("A", true))); // Shifted = true

        // Verify dialog text shows shift binding
        harness.get_by("Press a key to bind to \"Shift + A\" on the guest system.");
        harness.get_by("No binding set yet.");
        harness.get_by("Cancel");
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

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Find the A key layout
        let a_key_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "A")
            .expect("A key layout should exist");

        // Calculate center position of A key
        let a_key_center_x = (a_key_layout.x + a_key_layout.width / 2) as f32 / 2.0;
        let a_key_center_y = (a_key_layout.y + SIZE / 2) as f32 / 2.0;
        let a_key_pos = egui::Pos2::new(a_key_center_x, a_key_center_y);

        // Open binding dialog
        keyboard_image.hover_at(a_key_pos);
        harness.run();
        keyboard_image.click_at(a_key_pos);
        harness.run();

        // Verify dialog is open
        assert_eq!(key_map_editor.listening, Some(("A", false)));
        harness.get_by("Press a key to bind to \"A\" on the guest system.");

        // Click Cancel button
        harness.get_by("Cancel").click();
        harness.run();

        // Verify dialog is closed
        assert_eq!(key_map_editor.listening, None);

        // Verify no binding was set
        assert_eq!(key_mapper.binding("A", false), None);

        // Verify dialog text is no longer present
        assert!(
            harness
                .try_get_by("Press a key to bind to \"A\" on the guest system.")
                .is_none()
        );
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

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Find the A key layout
        let a_key_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "A")
            .expect("A key layout should exist");

        // Calculate center position of A key
        let a_key_center_x = (a_key_layout.x + a_key_layout.width / 2) as f32 / 2.0;
        let a_key_center_y = (a_key_layout.y + SIZE / 2) as f32 / 2.0;
        let a_key_pos = egui::Pos2::new(a_key_center_x, a_key_center_y);

        // Open binding dialog
        keyboard_image.hover_at(a_key_pos);
        harness.run();
        keyboard_image.click_at(a_key_pos);
        harness.run();

        // Verify existing binding is shown
        harness.get_by("Currently bound to Q");

        // Click Clear Binding button
        harness.get_by("Clear Binding").click();
        harness.run();

        // Verify binding was cleared
        assert_eq!(key_mapper.binding("A", false), None);

        // Verify dialog shows no binding
        harness.get_by("No binding set yet.");
        assert!(harness.try_get_by("Currently bound to Q").is_none());
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

        // Verify that Shift keys are excluded from the rendered keyboard layout
        // The code explicitly skips rendering keys with name "Shift" in the SVG generation

        // Count how many Shift keys are in the key_layouts (should be 2: left and right)
        let shift_keys_in_layouts = key_map_editor
            .key_layouts
            .iter()
            .filter(|layout| layout.name == "Shift")
            .count();
        assert_eq!(shift_keys_in_layouts, 2); // Both left and right shift exist in layouts

        // Verify the instruction text mentions that Shift keys cannot be bound
        harness.get_by("Click keys to set bindings. Shift-click to set bindings for shifted keys. The guest system's Shift keys themselves cannot be bound.");

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Test left Shift key (x=0, y=300)
        let left_shift_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "Shift" && layout.x == 0)
            .expect("Left Shift key layout should exist");

        let left_shift_center_x = (left_shift_layout.x + left_shift_layout.width / 2) as f32 / 2.0;
        let left_shift_center_y = (left_shift_layout.y + SIZE / 2) as f32 / 2.0;
        let left_shift_pos = egui::Pos2::new(left_shift_center_x, left_shift_center_y);

        keyboard_image.hover_at(left_shift_pos);
        harness.run();

        // Verify that hovering over left Shift key area doesn't set hovered_key
        assert_ne!(key_map_editor.hovered_key, Some("Shift"));

        // Test right Shift key (x=1300, y=300)
        let right_shift_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "Shift" && layout.x == 1300)
            .expect("Right Shift key layout should exist");

        let right_shift_center_x =
            (right_shift_layout.x + right_shift_layout.width / 2) as f32 / 2.0;
        let right_shift_center_y = (right_shift_layout.y + SIZE / 2) as f32 / 2.0;
        let right_shift_pos = egui::Pos2::new(right_shift_center_x, right_shift_center_y);

        keyboard_image.hover_at(right_shift_pos);
        harness.run();

        // Verify that hovering over right Shift key area doesn't set hovered_key
        assert_ne!(key_map_editor.hovered_key, Some("Shift"));
    }

    #[test]
    fn test_enter_key_special_shape_hit_detection() {
        let mut key_map_editor = KeyMapEditor::default();
        key_map_editor.show = true;
        let mut key_mapper = KeyMapper::<MockKeyMapStore>::default();

        let app = move |ctx: &egui::Context| {
            key_map_editor.ui(ctx, &mut key_mapper);
        };

        let mut harness = Harness::new(app);
        harness.run();

        // Get the keyboard layout image
        let keyboard_image = harness.get_by_role(Role::Image);

        // Find the Enter key layout
        let enter_key_layout = key_map_editor
            .key_layouts
            .iter()
            .find(|layout| layout.name == "Enter")
            .expect("Enter key layout should exist");

        // Test hit detection in the wide upper section of Enter key
        let upper_section_x = (enter_key_layout.x + PADDING + 50) as f32 / 2.0; // Well within upper section
        let upper_section_y = (enter_key_layout.y + PADDING + 20) as f32 / 2.0; // Upper part
        let upper_pos = egui::Pos2::new(upper_section_x, upper_section_y);

        keyboard_image.hover_at(upper_pos);
        harness.run();

        // Verify Enter key is detected in upper section
        assert_eq!(key_map_editor.hovered_key, Some("Enter"));

        // Test hit detection in the narrow lower section of Enter key
        let lower_section_x = (enter_key_layout.x + PADDING + SIZE / 4 + 20) as f32 / 2.0; // Within narrow section
        let lower_section_y = (enter_key_layout.y + SIZE + 20) as f32 / 2.0; // Lower part
        let lower_pos = egui::Pos2::new(lower_section_x, lower_section_y);

        keyboard_image.hover_at(lower_pos);
        harness.run();

        // Verify Enter key is still detected in narrow lower section
        assert_eq!(key_map_editor.hovered_key, Some("Enter"));

        // Test hit detection outside the narrow lower section (should miss)
        let outside_narrow_x = (enter_key_layout.x + PADDING + 10) as f32 / 2.0; // Too far left for narrow section
        let outside_narrow_y = (enter_key_layout.y + SIZE + 20) as f32 / 2.0; // Lower part
        let outside_pos = egui::Pos2::new(outside_narrow_x, outside_narrow_y);

        keyboard_image.hover_at(outside_pos);
        harness.run();

        // Verify Enter key is NOT detected outside its L-shape
        assert_ne!(key_map_editor.hovered_key, Some("Enter"));

        // Test clicking on Enter key in valid area
        keyboard_image.hover_at(upper_pos);
        harness.run();
        keyboard_image.click_at(upper_pos);
        harness.run();

        // Verify binding dialog opened for Enter key
        assert_eq!(key_map_editor.listening, Some(("Enter", false)));
        harness.get_by("Press a key to bind to \"Enter\" on the guest system.");
    }
}
