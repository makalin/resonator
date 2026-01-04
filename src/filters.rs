#[derive(Clone, Debug)]
pub enum MidiFilter {
    VelocityScale { factor: f32 },
    VelocityLimit { min: u8, max: u8 },
    PitchShift { semitones: i8 },
    PitchLimit { min: u8, max: u8 },
    Transpose { semitones: i8 },
    Quantize { grid: u32 },
}

pub struct FilterChain {
    filters: Vec<MidiFilter>,
}

impl FilterChain {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, filter: MidiFilter) {
        self.filters.push(filter);
    }

    pub fn remove_filter(&mut self, index: usize) {
        if index < self.filters.len() {
            self.filters.remove(index);
        }
    }

    pub fn filters(&self) -> &[MidiFilter] {
        &self.filters
    }

    pub fn apply(&self, pitch: u8, velocity: u8) -> (u8, u8) {
        let mut p = pitch as i16;
        let mut v = velocity as f32;

        for filter in &self.filters {
            match filter {
                MidiFilter::VelocityScale { factor } => {
                    v = (v * factor).min(127.0).max(0.0);
                }
                MidiFilter::VelocityLimit { min, max } => {
                    v = v.min(*max as f32).max(*min as f32);
                }
                MidiFilter::PitchShift { semitones } => {
                    p = (p as i16 + *semitones as i16).max(0).min(127);
                }
                MidiFilter::PitchLimit { min, max } => {
                    p = p.max(*min as i16).min(*max as i16);
                }
                MidiFilter::Transpose { semitones } => {
                    p = (p as i16 + *semitones as i16).max(0).min(127);
                }
                MidiFilter::Quantize { .. } => {
                    // Quantization is applied during note placement, not here
                }
            }
        }

        (p as u8, v as u8)
    }
}

