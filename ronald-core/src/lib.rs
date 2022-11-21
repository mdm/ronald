pub mod constants;
pub mod system;

pub trait VideoSink {
    fn set_pixel(&self, x: usize, y: usize, r: u8, g: u8, b: u8);
    fn submit_frame(&self);
}

pub trait AudioSink {}

pub struct Driver<S>
where
    S: system::System,
{
    system: S,
}

impl<S> Driver<S>
where
    S: system::System,
{
    pub fn new() -> Self {
        Self { system: S::new() }
    }

    pub fn step(&mut self, usecs: usize, video: &mut impl VideoSink, audio: Option<&mut impl AudioSink>) {
        let frame_start = std::time::Instant::now();

        let mut elapsed_microseconds = 0;
        while elapsed_microseconds < usecs {
            // TODO: tie this to vsync instead of fixed value
            elapsed_microseconds += self.system.emulate() as usize;
        }

        video.submit_frame();
    }

    pub fn activate_debugger(&self) {
        todo!()
    }

    pub fn press_key(&self, line: usize, bit: u8) { // TODO: use key names here?
        todo!()
    }

    pub fn release_key(&self, line: usize, bit: u8) { // TODO: use key names here?
        todo!()
    }

    pub fn load_rom(&self, slot: usize, rom: Vec<u8>) {
        // TODO: Return result -> Err if slot unsuitable
        todo!()
    }

    pub fn save_rom(&self) -> Vec<u8> {
        todo!()
    }

    pub fn load_snapshot(&self, rom: Vec<u8>) {
        todo!()
    }

    pub fn save_snapshot(&self) -> Vec<u8> {
        todo!()
    }
}

impl<S> Default for Driver<S>
where
    S: system::System,
{
    fn default() -> Self {
        Self::new()
    }
}
