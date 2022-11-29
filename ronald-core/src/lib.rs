pub mod constants;
pub mod system;

pub trait VideoSink {
    fn draw_frame(&mut self, buffer: &Vec<(u8, u8, u8)>);
}

pub trait AudioSink {
    fn get_sample_rate(&self) -> Option<f32>;
    fn play_audio(&self);
    fn pause_audio(&self);
    fn add_sample(&self, sample: f32);
}

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

    pub fn step(&mut self, usecs: usize, video: &mut impl VideoSink, audio: &mut impl AudioSink) {
        let mut elapsed_microseconds = 0;
        while elapsed_microseconds < usecs {
            // TODO: tie this to vsync instead of fixed value
            elapsed_microseconds += self.system.emulate(video, audio) as usize;
        }
    }

    pub fn activate_debugger(&self) {
        todo!()
    }

    pub fn press_key(&mut self, line: usize, bit: u8) { // TODO: use key names here?
        self.system.set_key(line, bit);
    }

    pub fn release_key(&mut self, line: usize, bit: u8) { // TODO: use key names here?
        self.system.unset_key(line, bit);
    }

    pub fn load_disk(&mut self, drive: usize, rom: Vec<u8>) {
        // TODO: Return result -> Err if slot unsuitable
        self.system.load_disk(drive, rom)
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
