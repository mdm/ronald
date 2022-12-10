use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use ronald_core::{system, Driver, VideoSink, AudioSink};

mod io;

use io::{CanvasVideo, WebAudio};

#[wasm_bindgen]
pub fn run_cpc(canvas: JsValue) -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let audio = WebAudio {};

    let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    let video = CanvasVideo::new(ctx);

    let driver = Driver::<system::CPC464>::new();

    step_driver(driver, video, audio);

    Ok(())
}

fn step_driver<S, V, A>(mut driver: Driver<S>, mut video: V, mut audio: A)
where
    S: system::System + 'static,
    V: VideoSink + 'static,
    A: AudioSink + 'static,
{
    let timer = gloo::console::Timer::new("frame");
    driver.step(20_000, &mut video, &mut audio);
    drop(timer);

    let wait_ms = 1_000 / 20;
    let timeout = Timeout::new(0, move || {
        step_driver(driver, video, audio);
    });

    timeout.forget();
}
