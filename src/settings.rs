use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub midi_output_port: Option<usize>,
    pub default_bpm: f64,
    pub default_time_signature: (u8, u8),
    pub default_velocity: u8,
    pub default_note_length: u32,
    pub auto_save: bool,
    pub auto_save_interval: u32, // seconds
    pub theme: Theme,
    pub piano_roll_zoom: u32,
    pub step_sequencer_steps: usize,
    pub step_sequencer_steps_per_beat: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Default,
    Dark,
    Light,
    HighContrast,
    Retro,
    Amber,
    Matrix,
    C64,
    Apple2,
    Monochrome,
    Neon,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            midi_output_port: None,
            default_bpm: 120.0,
            default_time_signature: (4, 4),
            default_velocity: 100,
            default_note_length: 480,
            auto_save: false,
            auto_save_interval: 300,
            theme: Theme::Default,
            piano_roll_zoom: 10,
            step_sequencer_steps: 16,
            step_sequencer_steps_per_beat: 4,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string("resonator.json") {
            if let Ok(settings) = serde_json::from_str(&content) {
                return settings;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("resonator.json", json)?;
        Ok(())
    }
}

