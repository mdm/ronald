use std::path::PathBuf;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use ronald_core::{system, AudioSink, Driver};

mod io;

use io::{CanvasVideo, WebAudio};

#[wasm_bindgen]
pub struct Emulator {
    driver: Driver<system::AmstradCpc464>,
    video: CanvasVideo,
    audio: WebAudio,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: JsValue) -> Result<Emulator, JsValue> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_logger::init(wasm_logger::Config::default());

        let audio = WebAudio::new();

        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        let video = CanvasVideo::new(ctx);

        let driver = Driver::<system::AmstradCpc464>::new();
        log::debug!("Initialized CPC 464 emulator");

        Ok(Self {
            driver,
            video,
            audio,
        })
    }

    pub fn step_driver(&mut self) {
        // TODO: allow stepping just for the time actually available
        self.driver.step(20_000, &mut self.video, &mut self.audio);
    }

    pub fn step_single(&mut self) {
        self.driver.step_single(&mut self.video, &mut self.audio);
    }

    pub fn press_key(&mut self, key: &str) {
        self.driver.press_key(key);
    }

    pub fn release_key(&mut self, key: &str) {
        self.driver.release_key(key);
    }

    pub fn load_disk(&mut self, drive: usize, rom: JsValue, path: &str) {
        let array = Uint8Array::new(&rom);
        self.driver
            .load_disk(drive, array.to_vec(), PathBuf::from(path));
    }

    pub fn play_audio(&mut self) {
        self.audio.play_audio()
    }

    pub fn pause_audio(&mut self) {
        self.audio.pause_audio()
    }

    pub fn get_snapshot(&self) -> String {
        self.driver.get_json_snapshot().unwrap_or_default()
    }

    pub fn disassemble(&mut self, count: usize) -> String {
        self.driver.disassemble(count).unwrap_or_default()
    }
}
