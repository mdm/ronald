use std::fmt::Display;

use eframe::egui;

const SIZE: usize = 100; // TODO: use this in GuestKey definitions
const PADDING: usize = 8;
const CORNER_RADIUS: usize = 10;
const STROKE_WIDTH: usize = 3;

struct GuestKey {
    name: &'static str,
    x: usize,
    y: usize,
    width: usize,
    label: &'static str,
}

impl GuestKey {
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

impl Display for GuestKey {
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

pub struct Keyboard {
    pub show: bool,
    hovered_key: Option<&'static str>,
    keys: Vec<GuestKey>,
}

impl Keyboard {
    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        let modal = egui::Modal::new("key_bindings_modal".into()).show(ctx, |ui| {
            ui.label("Key Bindings");

            let mut svg = String::new();
            svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2200 500">"#);
            for key in &self.keys {
                match self.hovered_key {
                    Some(hovered_key) if hovered_key == key.name => {
                        svg.push_str(r#"<g stroke="white" fill="white">"#);
                        svg.push_str(key.to_string().as_str());
                        svg.push_str(r#"</g>"#);
                        svg.push_str(r#"<g stroke=""#);
                        svg.push_str(&egui::style::Widgets::dark().noninteractive.bg_fill.to_hex());
                        svg.push_str(r#"" fill=""#);
                        svg.push_str(&egui::style::Widgets::dark().noninteractive.bg_fill.to_hex());
                        svg.push_str(r#"">"#);
                        svg.push_str(key.label);
                        svg.push_str(r#"</g>"#);
                    }
                    _ => {
                        svg.push_str(r#"<g stroke="white" fill="transparent">"#);
                        svg.push_str(key.to_string().as_str());
                        svg.push_str(r#"</g>"#);
                        svg.push_str(r#"<g stroke="white" fill="white">"#);
                        svg.push_str(key.label);
                        svg.push_str(r#"</g>"#);
                    }
                }
            }
            svg.push_str(r#"<g stroke="white" fill="white">"#);
            svg.push_str(include_str!("../assets/keys/JoystickIcon.partial.svg"));
            svg.push_str(r#"</g>"#);

            svg.push_str(r#"</svg>"#);
            let response = ui.add(
                egui::Image::new(egui::ImageSource::Bytes {
                    uri: "bytes://keyboard_layout.svg".into(),
                    bytes: svg.into_bytes().into(),
                })
                .fit_to_exact_size(egui::vec2(1100.0, 250.0)),
            );
            if let Some(pos) = ctx.pointer_hover_pos() {
                let pos = pos - response.rect.left_top();
                let pos = egui::Pos2::new(2.0 * pos.x, 2.0 * pos.y);
                let mut hovering = false;
                for key in &self.keys {
                    if key.contains_pos(pos) {
                        hovering = true;
                        match self.hovered_key {
                            Some(hovered_key) if hovered_key == key.name => {
                                // Already hovered, do nothing
                            }
                            _ => {
                                self.hovered_key = Some(key.name);
                                ctx.forget_image("bytes://keyboard_layout.svg");
                            }
                        }
                    }
                }

                if !hovering {
                    self.hovered_key = None;
                    ctx.forget_image("bytes://keyboard_layout.svg");
                }
            };

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Close").clicked() {
                    self.show = false;
                }
            });
        });
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            show: false,
            hovered_key: None,
            keys: vec![
                GuestKey {
                    name: "Escape",
                    x: 0,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Escape.partial.svg"),
                },
                GuestKey {
                    name: "Key1",
                    x: 100,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key1.partial.svg"),
                },
                GuestKey {
                    name: "Key2",
                    x: 200,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key2.partial.svg"),
                },
                GuestKey {
                    name: "Key3",
                    x: 300,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key3.partial.svg"),
                },
                GuestKey {
                    name: "Key4",
                    x: 400,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key4.partial.svg"),
                },
                GuestKey {
                    name: "Key5",
                    x: 500,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key5.partial.svg"),
                },
                GuestKey {
                    name: "Key6",
                    x: 600,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key6.partial.svg"),
                },
                GuestKey {
                    name: "Key7",
                    x: 700,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key7.partial.svg"),
                },
                GuestKey {
                    name: "Key8",
                    x: 800,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key8.partial.svg"),
                },
                GuestKey {
                    name: "Key9",
                    x: 900,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key9.partial.svg"),
                },
                GuestKey {
                    name: "Key0",
                    x: 1000,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Key0.partial.svg"),
                },
                GuestKey {
                    name: "Minus",
                    x: 1100,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Minus.partial.svg"),
                },
                GuestKey {
                    name: "Caret",
                    x: 1200,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Caret.partial.svg"),
                },
                GuestKey {
                    name: "Clear",
                    x: 1300,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Clear.partial.svg"),
                },
                GuestKey {
                    name: "Delete",
                    x: 1400,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Delete.partial.svg"),
                },
                GuestKey {
                    name: "Numpad7",
                    x: 1500,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad7.partial.svg"),
                },
                GuestKey {
                    name: "Numpad8",
                    x: 1600,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad8.partial.svg"),
                },
                GuestKey {
                    name: "Numpad9",
                    x: 1700,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad9.partial.svg"),
                },
                GuestKey {
                    name: "JoystickUp",
                    x: 2000,
                    y: 0,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickUp.partial.svg"),
                },
                GuestKey {
                    name: "Tab",
                    x: 0,
                    y: 100,
                    width: 125,
                    label: include_str!("../assets/keys/Tab.partial.svg"),
                },
                GuestKey {
                    name: "Q",
                    x: 125,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Q.partial.svg"),
                },
                GuestKey {
                    name: "W",
                    x: 225,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/W.partial.svg"),
                },
                GuestKey {
                    name: "E",
                    x: 325,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/E.partial.svg"),
                },
                GuestKey {
                    name: "R",
                    x: 425,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/R.partial.svg"),
                },
                GuestKey {
                    name: "T",
                    x: 525,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/T.partial.svg"),
                },
                GuestKey {
                    name: "Y",
                    x: 625,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Y.partial.svg"),
                },
                GuestKey {
                    name: "U",
                    x: 725,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/U.partial.svg"),
                },
                GuestKey {
                    name: "I",
                    x: 825,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/I.partial.svg"),
                },
                GuestKey {
                    name: "O",
                    x: 925,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/O.partial.svg"),
                },
                GuestKey {
                    name: "P",
                    x: 1025,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/P.partial.svg"),
                },
                GuestKey {
                    name: "At",
                    x: 1125,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/At.partial.svg"),
                },
                GuestKey {
                    name: "BracketLeft",
                    x: 1225,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/BracketLeft.partial.svg"),
                },
                GuestKey {
                    name: "Enter",
                    x: 1325,
                    y: 100,
                    width: 175,
                    label: include_str!("../assets/keys/Enter.partial.svg"),
                },
                GuestKey {
                    name: "Numpad4",
                    x: 1500,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad4.partial.svg"),
                },
                GuestKey {
                    name: "Numpad5",
                    x: 1600,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad5.partial.svg"),
                },
                GuestKey {
                    name: "Numpad6",
                    x: 1700,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad6.partial.svg"),
                },
                GuestKey {
                    name: "JoystickLeft",
                    x: 1900,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickLeft.partial.svg"),
                },
                GuestKey {
                    name: "JoystickRight",
                    x: 2100,
                    y: 100,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickRight.partial.svg"),
                },
                GuestKey {
                    name: "CapsLock",
                    x: 0,
                    y: 200,
                    width: 150,
                    label: include_str!("../assets/keys/CapsLock.partial.svg"),
                },
                GuestKey {
                    name: "A",
                    x: 150,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/A.partial.svg"),
                },
                GuestKey {
                    name: "S",
                    x: 250,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/S.partial.svg"),
                },
                GuestKey {
                    name: "D",
                    x: 350,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/D.partial.svg"),
                },
                GuestKey {
                    name: "F",
                    x: 450,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/F.partial.svg"),
                },
                GuestKey {
                    name: "G",
                    x: 550,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/G.partial.svg"),
                },
                GuestKey {
                    name: "H",
                    x: 650,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/H.partial.svg"),
                },
                GuestKey {
                    name: "J",
                    x: 750,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/J.partial.svg"),
                },
                GuestKey {
                    name: "K",
                    x: 850,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/K.partial.svg"),
                },
                GuestKey {
                    name: "L",
                    x: 950,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/L.partial.svg"),
                },
                GuestKey {
                    name: "Colon",
                    x: 1050,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Colon.partial.svg"),
                },
                GuestKey {
                    name: "Semicolon",
                    x: 1150,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Semicolon.partial.svg"),
                },
                GuestKey {
                    name: "BracketRight",
                    x: 1250,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/BracketRight.partial.svg"),
                },
                GuestKey {
                    name: "Numpad1",
                    x: 1500,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad1.partial.svg"),
                },
                GuestKey {
                    name: "Numpad2",
                    x: 1600,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad2.partial.svg"),
                },
                GuestKey {
                    name: "Numpad3",
                    x: 1700,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad3.partial.svg"),
                },
                GuestKey {
                    name: "JoystickDown",
                    x: 2000,
                    y: 200,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickDown.partial.svg"),
                },
                GuestKey {
                    name: "Shift",
                    x: 0,
                    y: 300,
                    width: 200,
                    label: include_str!("../assets/keys/ShiftLeft.partial.svg"),
                },
                GuestKey {
                    name: "Z",
                    x: 200,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Z.partial.svg"),
                },
                GuestKey {
                    name: "X",
                    x: 300,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/X.partial.svg"),
                },
                GuestKey {
                    name: "C",
                    x: 400,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/C.partial.svg"),
                },
                GuestKey {
                    name: "V",
                    x: 500,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/V.partial.svg"),
                },
                GuestKey {
                    name: "B",
                    x: 600,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/B.partial.svg"),
                },
                GuestKey {
                    name: "N",
                    x: 700,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/N.partial.svg"),
                },
                GuestKey {
                    name: "M",
                    x: 800,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/M.partial.svg"),
                },
                GuestKey {
                    name: "Comma",
                    x: 900,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Comma.partial.svg"),
                },
                GuestKey {
                    name: "Period",
                    x: 1000,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Period.partial.svg"),
                },
                GuestKey {
                    name: "Slash",
                    x: 1100,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Slash.partial.svg"),
                },
                GuestKey {
                    name: "Backslash",
                    x: 1200,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Backslash.partial.svg"),
                },
                GuestKey {
                    name: "Shift",
                    x: 1300,
                    y: 300,
                    width: 200,
                    label: include_str!("../assets/keys/ShiftRight.partial.svg"),
                },
                GuestKey {
                    name: "Numpad0",
                    x: 1500,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/Numpad0.partial.svg"),
                },
                GuestKey {
                    name: "ArrowUp",
                    x: 1600,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowUp.partial.svg"),
                },
                GuestKey {
                    name: "NumpadPeriod",
                    x: 1700,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/NumpadPeriod.partial.svg"),
                },
                GuestKey {
                    name: "JoystickFire1",
                    x: 1900,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickFire1.partial.svg"),
                },
                GuestKey {
                    name: "JoystickFire2",
                    x: 2000,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickFire2.partial.svg"),
                },
                GuestKey {
                    name: "JoystickFire3",
                    x: 2100,
                    y: 300,
                    width: 100,
                    label: include_str!("../assets/keys/JoystickFire3.partial.svg"),
                },
                GuestKey {
                    name: "Control",
                    x: 0,
                    y: 400,
                    width: 200,
                    label: include_str!("../assets/keys/Control.partial.svg"),
                },
                GuestKey {
                    name: "Copy",
                    x: 200,
                    y: 400,
                    width: 175,
                    label: include_str!("../assets/keys/Copy.partial.svg"),
                },
                GuestKey {
                    name: "Space",
                    x: 375,
                    y: 400,
                    width: 800,
                    label: "",
                },
                GuestKey {
                    name: "NumpadEnter",
                    x: 1175,
                    y: 400,
                    width: 325,
                    label: include_str!("../assets/keys/NumpadEnter.partial.svg"),
                },
                GuestKey {
                    name: "ArrowLeft",
                    x: 1500,
                    y: 400,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowLeft.partial.svg"),
                },
                GuestKey {
                    name: "ArrowDown",
                    x: 1600,
                    y: 400,
                    width: 100,
                    label: include_str!("../assets/keys/ArrowDown.partial.svg"),
                },
                GuestKey {
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
