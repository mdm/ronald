use ronald_egui::RonaldApp;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let scale_factor = 2.0;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0 * scale_factor, 400.0 * scale_factor])
            .with_min_inner_size([640.0 * scale_factor, 400.0 * scale_factor])
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
