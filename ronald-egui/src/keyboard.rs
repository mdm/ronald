use std::fmt::Display;

use eframe::egui;

const SIZE: usize = 100;
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
                q 0 -{CORNER_RADIUS} {CORNER_RADIUS} -{CORNER_RADIUS}" fill="transparent" stroke="white" stroke-width="{STROKE_WIDTH}"/>"#,
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
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" fill="transparent" stroke="white" stroke-width="{STROKE_WIDTH}"/>"#,
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
    keys: Vec<GuestKey>,
}

impl Keyboard {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Modal::new("key_bindings_modal".into()).show(ctx, |ui| {
            ui.label("Key Bindings");
            let mut svg = String::new();
            svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2200 500">"#);
            for key in &self.keys {
                svg.push_str(key.to_string().as_str());
            }
            svg.push_str(r#"</svg>"#);
            ui.add(
                egui::Image::new(egui::ImageSource::Bytes {
                    uri: "bytes://keyboard_layout.svg".into(),
                    bytes: svg.into_bytes().into(),
                })
                .fit_to_exact_size(egui::vec2(1100.0, 250.0)),
            );
        });
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            keys: vec![
                GuestKey {
                    name: "Escape",
                    x: 0,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key1",
                    x: 100,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key2",
                    x: 200,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key3",
                    x: 300,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key4",
                    x: 400,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key5",
                    x: 500,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key6",
                    x: 600,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key7",
                    x: 700,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key8",
                    x: 800,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key9",
                    x: 900,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Key0",
                    x: 1000,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Minus",
                    x: 1100,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Caret",
                    x: 1200,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Clear",
                    x: 1300,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Delete",
                    x: 1400,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad7",
                    x: 1500,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad8",
                    x: 1600,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad9",
                    x: 1700,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickUp",
                    x: 2000,
                    y: 0,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Tab",
                    x: 0,
                    y: 100,
                    width: 125,
                    label: "",
                },
                GuestKey {
                    name: "Q",
                    x: 125,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "W",
                    x: 225,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "E",
                    x: 325,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "R",
                    x: 425,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "T",
                    x: 525,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Y",
                    x: 625,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "U",
                    x: 725,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "I",
                    x: 825,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "O",
                    x: 925,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "P",
                    x: 1025,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "At",
                    x: 1125,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "BracketLeft",
                    x: 1225,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Enter",
                    x: 1325,
                    y: 100,
                    width: 175,
                    label: "",
                },
                GuestKey {
                    name: "Numpad4",
                    x: 1500,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad5",
                    x: 1600,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad6",
                    x: 1700,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickLeft",
                    x: 1900,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickRight",
                    x: 2100,
                    y: 100,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "CapsLock",
                    x: 0,
                    y: 200,
                    width: 150,
                    label: "",
                },
                GuestKey {
                    name: "A",
                    x: 150,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "S",
                    x: 250,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "D",
                    x: 350,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "F",
                    x: 450,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "G",
                    x: 550,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "H",
                    x: 650,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "J",
                    x: 750,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "K",
                    x: 850,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "L",
                    x: 950,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Colon",
                    x: 1050,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Semicolon",
                    x: 1150,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "BracketRight",
                    x: 1250,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad1",
                    x: 1500,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad2",
                    x: 1600,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Numpad3",
                    x: 1700,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickDown",
                    x: 2000,
                    y: 200,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Shift",
                    x: 0,
                    y: 300,
                    width: 200,
                    label: "",
                },
                GuestKey {
                    name: "Z",
                    x: 200,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "X",
                    x: 300,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "C",
                    x: 400,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "V",
                    x: 500,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "B",
                    x: 600,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "N",
                    x: 700,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "M",
                    x: 800,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Comma",
                    x: 900,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Period",
                    x: 1000,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Slash",
                    x: 1100,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Backslash",
                    x: 1200,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Shift",
                    x: 1300,
                    y: 300,
                    width: 200,
                    label: "",
                },
                GuestKey {
                    name: "Numpad0",
                    x: 1500,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "ArrowUp",
                    x: 1600,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "NumpadPeriod",
                    x: 1700,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickFire1",
                    x: 1900,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickFire2",
                    x: 2000,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "JoystickFire3",
                    x: 2100,
                    y: 300,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "Control",
                    x: 0,
                    y: 400,
                    width: 200,
                    label: "",
                },
                GuestKey {
                    name: "Copy",
                    x: 200,
                    y: 400,
                    width: 175,
                    label: "",
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
                    label: "",
                },
                GuestKey {
                    name: "ArrowLeft",
                    x: 1500,
                    y: 400,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "ArrowDown",
                    x: 1600,
                    y: 400,
                    width: 100,
                    label: "",
                },
                GuestKey {
                    name: "ArrowRight",
                    x: 1700,
                    y: 400,
                    width: 100,
                    label: "",
                },
            ],
        }
    }
}
