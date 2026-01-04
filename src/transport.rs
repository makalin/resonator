pub struct Transport {
    playing: bool,
    bpm: f64,
    time_signature_numerator: u8,
    time_signature_denominator: u8,
    loop_start: u32,
    loop_end: u32,
}

impl Transport {
    pub fn new() -> Self {
        Self {
            playing: false,
            bpm: 120.0,
            time_signature_numerator: 4,
            time_signature_denominator: 4,
            loop_start: 0,
            loop_end: 16 * 480, // 16 beats at 480 ticks per beat
        }
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn toggle_play(&mut self) {
        self.playing = !self.playing;
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn bpm(&self) -> f64 {
        self.bpm
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm.max(20.0).min(300.0);
    }

    pub fn time_signature(&self) -> (u8, u8) {
        (self.time_signature_numerator, self.time_signature_denominator)
    }

    pub fn set_time_signature(&mut self, numerator: u8, denominator: u8) {
        self.time_signature_numerator = numerator;
        self.time_signature_denominator = denominator;
    }

    pub fn loop_range(&self) -> (u32, u32) {
        (self.loop_start, self.loop_end)
    }

    pub fn set_loop_range(&mut self, start: u32, end: u32) {
        self.loop_start = start;
        self.loop_end = end;
    }
}

