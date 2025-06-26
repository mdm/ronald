use ronald_egui::{RonaldApp, SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

const SCALE_FACTOR: f32 = 2.0;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                SCREEN_BUFFER_WIDTH as f32 * SCALE_FACTOR,
                SCREEN_BUFFER_HEIGHT as f32 * SCALE_FACTOR,
            ])
            .with_min_inner_size([
                SCREEN_BUFFER_WIDTH as f32 * SCALE_FACTOR,
                SCREEN_BUFFER_HEIGHT as f32 * SCALE_FACTOR,
            ])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Ronald - An Amstrad CPC Emulator",
        native_options,
        Box::new(|cc| Ok(Box::new(RonaldApp::new(cc)))),
    )
}
