mod app;
mod frontend;
mod key_map_editor;
mod key_mapper;
mod system_config;
mod utils;

use app::{RonaldApp, SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

#[cfg(not(target_arch = "wasm32"))]
use key_mapper::NativeKeyMapStore;
#[cfg(target_arch = "wasm32")]
use key_mapper::WebKeyMapStore;

const SCALE_FACTOR: f32 = 1.5;

#[cfg(not(target_arch = "wasm32"))]
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
            )
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Ronald - An Amstrad CPC Emulator",
        native_options,
        Box::new(|cc| Ok(Box::new(RonaldApp::<NativeKeyMapStore>::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(RonaldApp::<WebKeyMapStore>::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
