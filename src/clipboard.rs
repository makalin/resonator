use crate::sequencer::Note;

#[derive(Clone, Debug)]
pub struct Clipboard {
    notes: Vec<Note>,
    reference_tick: u32,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            reference_tick: 0,
        }
    }

    pub fn copy(&mut self, notes: Vec<Note>, reference_tick: u32) {
        self.notes = notes;
        self.reference_tick = reference_tick;
    }

    pub fn paste(&self, target_tick: u32) -> Vec<Note> {
        let offset = target_tick as i32 - self.reference_tick as i32;
        self.notes
            .iter()
            .map(|note| Note {
                pitch: note.pitch,
                start: (note.start as i32 + offset).max(0) as u32,
                length: note.length,
                velocity: note.velocity,
            })
            .collect()
    }

    pub fn has_data(&self) -> bool {
        !self.notes.is_empty()
    }

    pub fn clear(&mut self) {
        self.notes.clear();
        self.reference_tick = 0;
    }
}

