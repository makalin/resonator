use crate::sequencer::{Note, Sequencer, Track};

#[derive(Clone, Debug)]
pub enum HistoryAction {
    AddNote { track_idx: usize, note: Note },
    DeleteNote { track_idx: usize, note_idx: usize, note: Note },
    ModifyNote { track_idx: usize, note_idx: usize, old_note: Note, new_note: Note },
    AddTrack { track: Track },
    DeleteTrack { track_idx: usize, track: Track },
    ModifyTrack { track_idx: usize, old_name: String, new_name: String },
    MovePlayhead { old_pos: u32, new_pos: u32 },
}

pub struct History {
    undo_stack: Vec<HistoryAction>,
    redo_stack: Vec<HistoryAction>,
    max_history: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 100,
        }
    }

    pub fn push(&mut self, action: HistoryAction) {
        self.undo_stack.push(action);
        self.redo_stack.clear(); // Clear redo stack when new action is performed
        
        // Limit history size
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self, sequencer: &mut Sequencer) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            match action.clone() {
                HistoryAction::AddNote { track_idx, note } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        if let Some(pos) = track.notes.iter().position(|n| n.start == note.start && n.pitch == note.pitch) {
                            track.notes.remove(pos);
                        }
                    }
                    self.redo_stack.push(action);
                    true
                }
                HistoryAction::DeleteNote { track_idx, note_idx, note } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        track.notes.insert(note_idx, note);
                    }
                    self.redo_stack.push(action);
                    true
                }
                HistoryAction::ModifyNote { track_idx, note_idx, old_note, new_note } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        if let Some(n) = track.notes.get_mut(note_idx) {
                            *n = old_note.clone();
                        }
                    }
                    // Push reverse action
                    self.redo_stack.push(HistoryAction::ModifyNote {
                        track_idx,
                        note_idx,
                        old_note: new_note,
                        new_note: old_note,
                    });
                    true
                }
                HistoryAction::AddTrack { track: _ } => {
                    sequencer.tracks_mut().pop();
                    self.redo_stack.push(action);
                    true
                }
                HistoryAction::DeleteTrack { track_idx, track } => {
                    sequencer.tracks_mut().insert(track_idx, track);
                    self.redo_stack.push(action);
                    true
                }
                HistoryAction::ModifyTrack { track_idx, old_name, new_name } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        track.name = old_name.clone();
                    }
                    self.redo_stack.push(HistoryAction::ModifyTrack {
                        track_idx,
                        old_name: new_name,
                        new_name: old_name,
                    });
                    true
                }
                HistoryAction::MovePlayhead { old_pos, new_pos } => {
                    sequencer.move_playhead_to(old_pos);
                    self.redo_stack.push(HistoryAction::MovePlayhead {
                        old_pos: new_pos,
                        new_pos: old_pos,
                    });
                    true
                }
            }
        } else {
            false
        }
    }

    pub fn redo(&mut self, sequencer: &mut Sequencer) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            match action.clone() {
                HistoryAction::AddNote { track_idx, note } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        track.notes.push(note);
                    }
                    self.undo_stack.push(action);
                    true
                }
                HistoryAction::DeleteNote { track_idx, note_idx, note: _ } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        if note_idx < track.notes.len() {
                            track.notes.remove(note_idx);
                        }
                    }
                    self.undo_stack.push(action);
                    true
                }
                HistoryAction::ModifyNote { track_idx, note_idx, old_note: _, new_note } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        if let Some(n) = track.notes.get_mut(note_idx) {
                            *n = new_note;
                        }
                    }
                    self.undo_stack.push(action);
                    true
                }
                HistoryAction::AddTrack { track } => {
                    sequencer.tracks_mut().push(track);
                    self.undo_stack.push(action);
                    true
                }
                HistoryAction::DeleteTrack { track_idx, track: _ } => {
                    if track_idx < sequencer.tracks_mut().len() {
                        sequencer.tracks_mut().remove(track_idx);
                    }
                    self.undo_stack.push(action);
                    true
                }
                HistoryAction::ModifyTrack { track_idx, old_name: _, new_name } => {
                    if let Some(track) = sequencer.tracks_mut().get_mut(track_idx) {
                        track.name = new_name;
                    }
                    self.undo_stack.push(action);
                    true
                }
                HistoryAction::MovePlayhead { old_pos: _, new_pos } => {
                    sequencer.move_playhead_to(new_pos);
                    self.undo_stack.push(action);
                    true
                }
            }
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}

