use crate::midi::MidiManager;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Note {
    pub pitch: u8,
    pub start: u32,
    pub length: u32,
    pub velocity: u8,
}

#[derive(Clone, Debug)]
pub struct Track {
    pub name: String,
    pub notes: Vec<Note>,
    pub channel: u8,
    pub muted: bool,
}

pub struct Sequencer {
    tracks: Vec<Track>,
    selected_track: usize,
    selected_note: Option<(usize, usize)>, // (track_index, note_index)
    playhead: u32,
    current_tick: u32,
    ticks_per_beat: u32,
    active_notes: HashMap<(usize, u8), u32>, // (track_index, pitch) -> end_tick
    view_start_tick: u32,
    view_start_pitch: u8,
    zoom_level: u32, // ticks per pixel
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            selected_track: 0,
            selected_note: None,
            playhead: 0,
            current_tick: 0,
            ticks_per_beat: 480,
            active_notes: HashMap::new(),
            view_start_tick: 0,
            view_start_pitch: 36, // C2
            zoom_level: 10,
        }
    }

    pub fn add_track(&mut self, name: String) {
        self.tracks.push(Track {
            name,
            notes: Vec::new(),
            channel: self.tracks.len() as u8,
            muted: false,
        });
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    pub fn tracks_mut(&mut self) -> &mut Vec<Track> {
        &mut self.tracks
    }

    pub fn selected_track(&self) -> usize {
        self.selected_track
    }

    pub fn playhead(&self) -> u32 {
        self.playhead
    }

    pub fn current_tick(&self) -> u32 {
        self.current_tick
    }

    pub fn move_playhead(&mut self, delta: i32) {
        let new_pos = self.playhead as i32 + delta;
        self.playhead = new_pos.max(0) as u32;
    }

    pub fn select_next_track(&mut self) {
        if !self.tracks.is_empty() {
            self.selected_track = (self.selected_track + 1) % self.tracks.len();
        }
    }

    pub fn select_prev_track(&mut self) {
        if !self.tracks.is_empty() {
            self.selected_track = if self.selected_track == 0 {
                self.tracks.len() - 1
            } else {
                self.selected_track - 1
            };
        }
    }

    pub fn add_note_at_playhead(&mut self) {
        if self.selected_track < self.tracks.len() {
            let track = &mut self.tracks[self.selected_track];
            // Default note: middle C, 1 beat length, velocity 100
            track.notes.push(Note {
                pitch: 60,
                start: self.playhead,
                length: self.ticks_per_beat,
                velocity: 100,
            });
        }
    }

    pub fn delete_selected_note(&mut self) {
        if let Some((track_idx, note_idx)) = self.selected_note {
            if track_idx < self.tracks.len() {
                let track = &mut self.tracks[track_idx];
                if note_idx < track.notes.len() {
                    track.notes.remove(note_idx);
                    self.selected_note = None;
                }
            }
        } else if self.selected_track < self.tracks.len() {
            let track = &mut self.tracks[self.selected_track];
            if !track.notes.is_empty() {
                track.notes.pop();
            }
        }
    }

    pub fn selected_note(&self) -> Option<(usize, usize)> {
        self.selected_note
    }

    pub fn select_note(&mut self, track_idx: usize, note_idx: usize) {
        if track_idx < self.tracks.len() {
            let track = &self.tracks[track_idx];
            if note_idx < track.notes.len() {
                self.selected_note = Some((track_idx, note_idx));
            }
        }
    }

    pub fn get_note(&self, track_idx: usize, note_idx: usize) -> Option<&Note> {
        self.tracks.get(track_idx)?.notes.get(note_idx)
    }

    pub fn get_note_mut(&mut self, track_idx: usize, note_idx: usize) -> Option<&mut Note> {
        self.tracks.get_mut(track_idx)?.notes.get_mut(note_idx)
    }

    pub fn edit_note_pitch(&mut self, track_idx: usize, note_idx: usize, pitch: u8) -> bool {
        if let Some(note) = self.get_note_mut(track_idx, note_idx) {
            note.pitch = pitch.min(127);
            true
        } else {
            false
        }
    }

    pub fn edit_note_velocity(&mut self, track_idx: usize, note_idx: usize, velocity: u8) -> bool {
        if let Some(note) = self.get_note_mut(track_idx, note_idx) {
            note.velocity = velocity.min(127);
            true
        } else {
            false
        }
    }

    pub fn edit_note_length(&mut self, track_idx: usize, note_idx: usize, length: u32) -> bool {
        if let Some(note) = self.get_note_mut(track_idx, note_idx) {
            note.length = length.max(1);
            true
        } else {
            false
        }
    }

    pub fn edit_note_start(&mut self, track_idx: usize, note_idx: usize, start: u32) -> bool {
        if let Some(note) = self.get_note_mut(track_idx, note_idx) {
            note.start = start;
            true
        } else {
            false
        }
    }

    pub fn get_notes_in_range(&self, track_idx: usize, start_tick: u32, end_tick: u32) -> Vec<(usize, Note)> {
        if let Some(track) = self.tracks.get(track_idx) {
            track.notes
                .iter()
                .enumerate()
                .filter(|(_, note)| note.start >= start_tick && note.start < end_tick)
                .map(|(idx, note)| (idx, note.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn delete_track(&mut self, track_idx: usize) -> bool {
        if track_idx < self.tracks.len() && self.tracks.len() > 1 {
            self.tracks.remove(track_idx);
            if self.selected_track >= self.tracks.len() {
                self.selected_track = self.tracks.len() - 1;
            }
            true
        } else {
            false
        }
    }

    pub fn rename_track(&mut self, track_idx: usize, name: String) {
        if let Some(track) = self.tracks.get_mut(track_idx) {
            track.name = name;
        }
    }

    pub fn view_start_tick(&self) -> u32 {
        self.view_start_tick
    }

    pub fn view_start_pitch(&self) -> u8 {
        self.view_start_pitch
    }

    pub fn zoom_level(&self) -> u32 {
        self.zoom_level
    }

    pub fn set_view_start_tick(&mut self, tick: u32) {
        self.view_start_tick = tick;
    }

    pub fn set_view_start_pitch(&mut self, pitch: u8) {
        self.view_start_pitch = pitch.max(0).min(127);
    }

    pub fn set_zoom_level(&mut self, zoom: u32) {
        self.zoom_level = zoom.max(1).min(1000);
    }

    pub fn move_playhead_to(&mut self, tick: u32) {
        self.playhead = tick;
    }

    pub fn ticks_per_beat(&self) -> u32 {
        self.ticks_per_beat
    }

    pub fn tick(&mut self, bpm: f64, midi_manager: &MidiManager) {
        let ticks_per_second = (self.ticks_per_beat as f64 * bpm / 60.0) as u32;
        self.current_tick += ticks_per_second / 60; // Assuming 60 FPS update rate

        // Check for notes that should start
        for (track_idx, track) in self.tracks.iter().enumerate() {
            if track.muted {
                continue;
            }

            for note in &track.notes {
                if note.start == self.current_tick {
                    midi_manager.send_note_on(track.channel, note.pitch, note.velocity);
                    let end_tick = note.start + note.length;
                    self.active_notes.insert((track_idx, note.pitch), end_tick);
                }
            }
        }

        // Check for notes that should end
        let mut to_remove = Vec::new();
        for ((track_idx, pitch), &end_tick) in &self.active_notes {
            if self.current_tick >= end_tick {
                if let Some(track) = self.tracks.get(*track_idx) {
                    midi_manager.send_note_off(track.channel, *pitch);
                }
                to_remove.push((*track_idx, *pitch));
            }
        }

        for key in to_remove {
            self.active_notes.remove(&key);
        }
    }

    pub fn reset(&mut self) {
        self.current_tick = 0;
        self.active_notes.clear();
    }

    pub fn set_tracks(&mut self, tracks: Vec<Track>) {
        self.tracks = tracks;
    }

    pub fn set_playhead(&mut self, playhead: u32) {
        self.playhead = playhead;
    }
}

