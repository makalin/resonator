use anyhow::{Context, Result};
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};

pub struct MidiManager {
    output: Option<Arc<Mutex<MidiOutputConnection>>>,
}

impl MidiManager {
    pub fn new() -> Result<Self> {
        let output = MidiOutput::new("Resonator Output")
            .context("Failed to create MIDI output")?;
        
        // Try to connect to the first available output port
        let port_count = output.port_count();
        let connection = if port_count > 0 {
            match output.connect(0, "resonator") {
                Ok(conn) => Some(Arc::new(Mutex::new(conn))),
                Err(e) => {
                    eprintln!("Warning: Could not connect to MIDI output: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self { output: connection })
    }

    pub fn send_note_on(&self, channel: u8, note: u8, velocity: u8) {
        if let Some(conn) = &self.output {
            if let Ok(mut conn) = conn.lock() {
                let _ = conn.send(&[0x90 | channel, note, velocity]);
            }
        }
    }

    pub fn send_note_off(&self, channel: u8, note: u8) {
        if let Some(conn) = &self.output {
            if let Ok(mut conn) = conn.lock() {
                let _ = conn.send(&[0x80 | channel, note, 0]);
            }
        }
    }

    pub fn send_cc(&self, channel: u8, controller: u8, value: u8) {
        if let Some(conn) = &self.output {
            if let Ok(mut conn) = conn.lock() {
                let _ = conn.send(&[0xB0 | channel, controller, value]);
            }
        }
    }
}

