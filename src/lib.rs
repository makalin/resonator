pub mod clipboard;
pub mod filters;
pub mod history;
pub mod midi;
pub mod mixer;
pub mod project;
pub mod sequencer;
pub mod settings;
pub mod step_sequencer;
pub mod theme;
pub mod transport;
pub mod tui;

pub use app::App;

mod app {
    use crate::clipboard::Clipboard;
    use crate::filters::FilterChain;
    use crate::history::History;
    use crate::midi::MidiManager;
    use crate::mixer::Mixer;
    use crate::project::Project;
    use crate::sequencer::Sequencer;
    use crate::settings::Settings;
    use crate::step_sequencer::StepSequencer;
    use crate::transport::Transport;
    use crate::tui::Tui;
    use anyhow::Result;
    use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
    use ratatui::Terminal;
    use std::time::Duration;

    pub struct App {
        sequencer: Sequencer,
        step_sequencer: StepSequencer,
        mixer: Mixer,
        transport: Transport,
        midi_manager: MidiManager,
        project: Project,
        tui: Tui,
        history: History,
        clipboard: Clipboard,
        filters: FilterChain,
        settings: Settings,
        should_quit: bool,
        track_counter: usize,
        editing_note: Option<(usize, usize)>, // (track_idx, note_idx)
        edit_mode: EditMode,
    }

    #[derive(Clone, Copy, PartialEq)]
    enum EditMode {
        None,
        Pitch,
        Velocity,
        Length,
    }

    impl App {
        pub fn new() -> Result<Self> {
            let settings = Settings::load();
            let mut sequencer = Sequencer::new();
            let mut step_sequencer = StepSequencer::new(settings.step_sequencer_steps, settings.step_sequencer_steps_per_beat);
            let mixer = Mixer::new();
            let transport = Transport::new();
            let midi_manager = MidiManager::new()?;
            let project = Project::new();
            let tui = Tui::new();
            let history = History::new();
            let clipboard = Clipboard::new();
            let filters = FilterChain::new();

            // Initialize with default track
            sequencer.add_track("Track 1".to_string());
            step_sequencer.add_track("Step Track 1".to_string());

            // Initialize mixer tracks to match sequencer
            let mut mixer = Mixer::new();
            mixer.add_track(); // Add track for the default sequencer track

            Ok(Self {
                sequencer,
                step_sequencer,
                mixer,
                transport,
                midi_manager,
                project,
                tui,
                history,
                clipboard,
                filters,
                settings,
                should_quit: false,
                track_counter: 1,
                editing_note: None,
                edit_mode: EditMode::None,
            })
        }

        pub fn run(&mut self, mut terminal: Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) -> Result<()> {
            loop {
                terminal.draw(|f| {
                    self.tui.render(f, &self.sequencer, &self.step_sequencer, &self.mixer, &self.transport, &self.settings, &self.history, &self.clipboard)
                })?;

                if crossterm::event::poll(Duration::from_millis(16))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            self.handle_key_with_modifiers(key.code, key.modifiers)?;
                        }
                    }
                }

                // Update sequencer if playing
                if self.transport.is_playing() {
                    self.sequencer.tick(
                        self.transport.bpm(),
                        &self.midi_manager,
                    );
                    // Update step sequencer
                    self.step_sequencer.advance_step();
                }

                // Update TUI
                self.tui.tick();

                if self.should_quit {
                    break;
                }
            }

            Ok(())
        }

        fn handle_key(&mut self, key: KeyCode) -> Result<()> {
            // Handle quit confirmation first
            if self.tui.show_quit_confirm() {
                match key {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.should_quit = true;
                        self.tui.set_quit_confirm(false);
                        return Ok(());
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        self.tui.set_quit_confirm(false);
                        return Ok(());
                    }
                    _ => {
                        // Only allow Y/N/Esc when quit confirmation is shown
                        return Ok(());
                    }
                }
            }

            // Handle help overlay - it should work even when menu is open
            if self.tui.show_help() {
                match key {
                    KeyCode::Char('?') | KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Esc => {
                        self.tui.toggle_help();
                        return Ok(());
                    }
                    _ => {
                        // If help is open, only allow closing it
                        return Ok(());
                    }
                }
            }

            // Handle menu navigation
            if matches!(self.tui.menu_state(), crate::tui::MenuState::None) {
                match key {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        // Show quit confirmation dialog
                        self.tui.set_quit_confirm(true);
                    }
                    KeyCode::Char(' ') => {
                        self.transport.toggle_play();
                        self.tui.show_status(if self.transport.is_playing() {
                            "Playing".to_string()
                        } else {
                            "Stopped".to_string()
                        });
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.transport.stop();
                        self.sequencer.reset();
                        self.sequencer.move_playhead_to(0);
                        self.tui.show_status("Reset to start".to_string());
                    }
                    KeyCode::Left => {
                        let zoom = self.sequencer.zoom_level() as i32;
                        self.sequencer.move_playhead(-zoom);
                    }
                    KeyCode::Char('h') => {
                        // H key toggles help overlay
                        self.tui.toggle_help();
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        let zoom = self.sequencer.zoom_level() as i32;
                        self.sequencer.move_playhead(zoom);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.sequencer.select_next_track();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.sequencer.select_prev_track();
                    }
                    KeyCode::Insert | KeyCode::Enter => {
                        if self.edit_mode == EditMode::None {
                            self.sequencer.add_note_at_playhead();
                            if let Some(track) = self.sequencer.tracks().get(self.sequencer.selected_track()) {
                                if let Some(note) = track.notes.last() {
                                    self.history.push(crate::history::HistoryAction::AddNote {
                                        track_idx: self.sequencer.selected_track(),
                                        note: note.clone(),
                                    });
                                }
                            }
                            self.tui.show_status("Note added".to_string());
                        } else {
                            // In edit mode, confirm edit
                            self.edit_mode = EditMode::None;
                            self.editing_note = None;
                            self.tui.show_status("Edit confirmed".to_string());
                        }
                    }
                    KeyCode::Delete | KeyCode::Backspace => {
                        if let Some((track_idx, note_idx)) = self.sequencer.selected_note() {
                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                let note_clone = note.clone();
                                self.sequencer.delete_selected_note();
                                self.history.push(crate::history::HistoryAction::DeleteNote {
                                    track_idx,
                                    note_idx,
                                    note: note_clone,
                                });
                                self.tui.show_status("Note deleted".to_string());
                            }
                        } else {
                            self.sequencer.delete_selected_note();
                            self.tui.show_status("Note deleted".to_string());
                        }
                    }
                    KeyCode::F(2) => {
                        // F2 to enter edit mode
                        if let Some((track_idx, note_idx)) = self.sequencer.selected_note() {
                            self.editing_note = Some((track_idx, note_idx));
                            self.edit_mode = EditMode::Pitch;
                            self.tui.show_status("Edit mode: Pitch (P/W/L to switch, arrows to adjust)".to_string());
                        } else {
                            self.tui.show_status("No note selected".to_string());
                        }
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        if self.editing_note.is_some() {
                            self.edit_mode = EditMode::Pitch;
                            self.tui.show_status("Editing pitch".to_string());
                        }
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        if self.editing_note.is_some() {
                            self.edit_mode = EditMode::Velocity;
                            self.tui.show_status("Editing velocity".to_string());
                        } else {
                            // If not in edit mode, fall through to menu handler
                            self.tui.set_menu_state(crate::tui::MenuState::View);
                        }
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        if self.editing_note.is_some() {
                            self.edit_mode = EditMode::Length;
                            self.tui.show_status("Editing length".to_string());
                        }
                    }
                    KeyCode::Char('1') => {
                        self.tui.set_view(crate::tui::View::PianoRoll);
                    }
                    KeyCode::Char('2') => {
                        self.tui.set_view(crate::tui::View::Mixer);
                    }
                    KeyCode::Char('3') => {
                        self.tui.set_view(crate::tui::View::Settings);
                    }
                    KeyCode::Char('4') => {
                        self.tui.set_view(crate::tui::View::StepSequencer);
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        let new_zoom = (self.sequencer.zoom_level() * 2).min(1000);
                        self.sequencer.set_zoom_level(new_zoom);
                        self.tui.show_status(format!("Zoom: {}", new_zoom));
                    }
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        let new_zoom = (self.sequencer.zoom_level() / 2).max(1);
                        self.sequencer.set_zoom_level(new_zoom);
                        self.tui.show_status(format!("Zoom: {}", new_zoom));
                    }
                    KeyCode::Char('[') => {
                        let new_pitch = self.sequencer.view_start_pitch().saturating_sub(12);
                        self.sequencer.set_view_start_pitch(new_pitch);
                    }
                    KeyCode::Char(']') => {
                        let new_pitch = (self.sequencer.view_start_pitch() + 12).min(127);
                        self.sequencer.set_view_start_pitch(new_pitch);
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        self.tui.set_menu_state(crate::tui::MenuState::File);
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.tui.set_menu_state(crate::tui::MenuState::Edit);
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        // T key opens Track menu
                        self.tui.set_menu_state(crate::tui::MenuState::Track);
                    }
                    KeyCode::Char('v') | KeyCode::Char('V') => {
                        self.tui.set_menu_state(crate::tui::MenuState::View);
                    }
                    KeyCode::Char('?') => {
                        self.tui.toggle_help();
                    }
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        // H key toggles help overlay
                        self.tui.toggle_help();
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        if self.sequencer.tracks().get(self.sequencer.selected_track()).is_some() {
                            // Toggle mute - would need mutable access, handled in sequencer
                            self.tui.show_status("Mute toggled".to_string());
                        }
                    }
                    KeyCode::Char('`') | KeyCode::Char('~') => {
                        // Cycle themes - backtick/tilde key (works well on macOS)
                        self.settings.theme = match self.settings.theme {
                            crate::settings::Theme::Default => crate::settings::Theme::Retro,
                            crate::settings::Theme::Retro => crate::settings::Theme::Amber,
                            crate::settings::Theme::Amber => crate::settings::Theme::Matrix,
                            crate::settings::Theme::Matrix => crate::settings::Theme::C64,
                            crate::settings::Theme::C64 => crate::settings::Theme::Apple2,
                            crate::settings::Theme::Apple2 => crate::settings::Theme::Monochrome,
                            crate::settings::Theme::Monochrome => crate::settings::Theme::Neon,
                            crate::settings::Theme::Neon => crate::settings::Theme::Dark,
                            crate::settings::Theme::Dark => crate::settings::Theme::Light,
                            crate::settings::Theme::Light => crate::settings::Theme::HighContrast,
                            crate::settings::Theme::HighContrast => crate::settings::Theme::Default,
                        };
                        let _ = self.settings.save();
                        self.tui.show_status(format!("Theme: {:?}", self.settings.theme));
                    }
                    KeyCode::F(5) => {
                        // F5 to cycle themes (may require Fn key on macOS)
                        self.settings.theme = match self.settings.theme {
                            crate::settings::Theme::Default => crate::settings::Theme::Retro,
                            crate::settings::Theme::Retro => crate::settings::Theme::Amber,
                            crate::settings::Theme::Amber => crate::settings::Theme::Matrix,
                            crate::settings::Theme::Matrix => crate::settings::Theme::C64,
                            crate::settings::Theme::C64 => crate::settings::Theme::Apple2,
                            crate::settings::Theme::Apple2 => crate::settings::Theme::Monochrome,
                            crate::settings::Theme::Monochrome => crate::settings::Theme::Neon,
                            crate::settings::Theme::Neon => crate::settings::Theme::Dark,
                            crate::settings::Theme::Dark => crate::settings::Theme::Light,
                            crate::settings::Theme::Light => crate::settings::Theme::HighContrast,
                            crate::settings::Theme::HighContrast => crate::settings::Theme::Default,
                        };
                        let _ = self.settings.save();
                        self.tui.show_status(format!("Theme: {:?}", self.settings.theme));
                    }
                    KeyCode::Esc => {
                        self.tui.set_menu_state(crate::tui::MenuState::None);
                    }
                    _ => {}
                }
            } else {
                // Menu is open
                match key {
                    KeyCode::Esc => {
                        self.tui.set_menu_state(crate::tui::MenuState::None);
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        // Add track from Track menu
                        if matches!(self.tui.menu_state(), crate::tui::MenuState::Track) {
                            self.track_counter += 1;
                            self.sequencer.add_track(format!("Track {}", self.track_counter));
                            self.mixer.add_track();
                            self.tui.set_menu_state(crate::tui::MenuState::None);
                            self.tui.show_status(format!("Track {} added", self.track_counter));
                        } else {
                            self.tui.set_menu_state(crate::tui::MenuState::None);
                            self.handle_key(key)?;
                        }
                    }
                    _ => {
                        self.tui.set_menu_state(crate::tui::MenuState::None);
                        // Process the key as normal after closing menu
                        self.handle_key(key)?;
                    }
                }
            }
            Ok(())
        }

        pub fn handle_key_with_modifiers(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
            if modifiers.contains(KeyModifiers::CONTROL) {
                match key {
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        self.project.save(&self.sequencer, &self.mixer, &self.transport)?;
                        self.tui.show_status("Project saved".to_string());
                    }
                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        // Open project - would need file dialog, placeholder for now
                        self.tui.show_status("Open project (not implemented)".to_string());
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        // New project
                        *self = Self::new()?;
                        self.tui.show_status("New project created".to_string());
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        // Delete track
                        if self.sequencer.tracks().len() > 1 {
                            let track_idx = self.sequencer.selected_track();
                            if let Some(track) = self.sequencer.tracks().get(track_idx).cloned() {
                                if self.sequencer.delete_track(track_idx) {
                                    self.history.push(crate::history::HistoryAction::DeleteTrack {
                                        track_idx,
                                        track,
                                    });
                                    self.tui.show_status(format!("Track {} deleted", track_idx + 1));
                                }
                            }
                        }
                    }
                    KeyCode::Char('z') | KeyCode::Char('Z') => {
                        // Undo
                        if self.history.undo(&mut self.sequencer) {
                            self.tui.show_status("Undo".to_string());
                        } else {
                            self.tui.show_status("Nothing to undo".to_string());
                        }
                    }
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        // Redo
                        if self.history.redo(&mut self.sequencer) {
                            self.tui.show_status("Redo".to_string());
                        } else {
                            self.tui.show_status("Nothing to redo".to_string());
                        }
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        // Ctrl+T to cycle themes (alternative to backtick)
                        if modifiers.contains(KeyModifiers::SHIFT) {
                            // Cycle themes
                            self.settings.theme = match self.settings.theme {
                                crate::settings::Theme::Default => crate::settings::Theme::Retro,
                                crate::settings::Theme::Retro => crate::settings::Theme::Amber,
                                crate::settings::Theme::Amber => crate::settings::Theme::Matrix,
                                crate::settings::Theme::Matrix => crate::settings::Theme::C64,
                                crate::settings::Theme::C64 => crate::settings::Theme::Apple2,
                                crate::settings::Theme::Apple2 => crate::settings::Theme::Monochrome,
                                crate::settings::Theme::Monochrome => crate::settings::Theme::Neon,
                                crate::settings::Theme::Neon => crate::settings::Theme::Dark,
                                crate::settings::Theme::Dark => crate::settings::Theme::Light,
                                crate::settings::Theme::Light => crate::settings::Theme::HighContrast,
                                crate::settings::Theme::HighContrast => crate::settings::Theme::Default,
                            };
                            let _ = self.settings.save();
                            self.tui.show_status(format!("Theme: {:?}", self.settings.theme));
                        }
                        // Otherwise Ctrl+T is handled as delete track (existing behavior)
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        // Copy
                        if let Some((track_idx, _)) = self.sequencer.selected_note() {
                            let playhead = self.sequencer.playhead();
                            let notes = self.sequencer.get_notes_in_range(track_idx, playhead.saturating_sub(100), playhead + 100);
                            if !notes.is_empty() {
                                let note_refs: Vec<_> = notes.iter().map(|(_, n)| n.clone()).collect();
                                self.clipboard.copy(note_refs, playhead);
                                self.tui.show_status(format!("Copied {} notes", notes.len()));
                            } else {
                                self.tui.show_status("No notes to copy".to_string());
                            }
                        } else {
                            self.tui.show_status("No selection to copy".to_string());
                        }
                    }
                    KeyCode::Char('v') | KeyCode::Char('V') => {
                        // Paste
                        if self.clipboard.has_data() {
                            let playhead = self.sequencer.playhead();
                            let notes = self.clipboard.paste(playhead);
                            let track_idx = self.sequencer.selected_track();
                            for note in &notes {
                                if let Some(track) = self.sequencer.tracks_mut().get_mut(track_idx) {
                                    track.notes.push(note.clone());
                                    self.history.push(crate::history::HistoryAction::AddNote {
                                        track_idx,
                                        note: note.clone(),
                                    });
                                }
                            }
                            self.tui.show_status(format!("Pasted {} notes", notes.len()));
                        } else {
                            self.tui.show_status("Clipboard is empty".to_string());
                        }
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        // Cut
                        if let Some((track_idx, note_idx)) = self.sequencer.selected_note() {
                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                let note_clone = note.clone();
                                let playhead = self.sequencer.playhead();
                                self.clipboard.copy(vec![note_clone.clone()], playhead);
                                self.sequencer.delete_selected_note();
                                self.history.push(crate::history::HistoryAction::DeleteNote {
                                    track_idx,
                                    note_idx,
                                    note: note_clone,
                                });
                                self.tui.show_status("Cut note".to_string());
                            }
                        }
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        // Select all - placeholder
                        self.tui.show_status("Select all (not fully implemented)".to_string());
                    }
                    _ => {
                        self.handle_key(key)?;
                    }
                }
            } else {
                // Handle edit mode adjustments
                if self.edit_mode != EditMode::None && self.editing_note.is_some() {
                    if let Some((track_idx, note_idx)) = self.editing_note {
                        match key {
                            KeyCode::Up => {
                                match self.edit_mode {
                                    EditMode::Pitch => {
                                        if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                            let old_note = note.clone();
                                            let new_pitch = (note.pitch + 1).min(127);
                                            self.sequencer.edit_note_pitch(track_idx, note_idx, new_pitch);
                                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                                self.history.push(crate::history::HistoryAction::ModifyNote {
                                                    track_idx,
                                                    note_idx,
                                                    old_note,
                                                    new_note: note.clone(),
                                                });
                                            }
                                        }
                                    }
                                    EditMode::Velocity => {
                                        if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                            let old_note = note.clone();
                                            let new_velocity = (note.velocity + 1).min(127);
                                            self.sequencer.edit_note_velocity(track_idx, note_idx, new_velocity);
                                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                                self.history.push(crate::history::HistoryAction::ModifyNote {
                                                    track_idx,
                                                    note_idx,
                                                    old_note,
                                                    new_note: note.clone(),
                                                });
                                            }
                                        }
                                    }
                                    EditMode::Length => {
                                        if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                            let old_note = note.clone();
                                            let new_length = note.length + self.sequencer.ticks_per_beat() / 4;
                                            self.sequencer.edit_note_length(track_idx, note_idx, new_length);
                                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                                self.history.push(crate::history::HistoryAction::ModifyNote {
                                                    track_idx,
                                                    note_idx,
                                                    old_note,
                                                    new_note: note.clone(),
                                                });
                                            }
                                        }
                                    }
                                    EditMode::None => {}
                                }
                            }
                            KeyCode::Down => {
                                match self.edit_mode {
                                    EditMode::Pitch => {
                                        if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                            let old_note = note.clone();
                                            let new_pitch = note.pitch.saturating_sub(1);
                                            self.sequencer.edit_note_pitch(track_idx, note_idx, new_pitch);
                                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                                self.history.push(crate::history::HistoryAction::ModifyNote {
                                                    track_idx,
                                                    note_idx,
                                                    old_note,
                                                    new_note: note.clone(),
                                                });
                                            }
                                        }
                                    }
                                    EditMode::Velocity => {
                                        if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                            let old_note = note.clone();
                                            let new_velocity = note.velocity.saturating_sub(1);
                                            self.sequencer.edit_note_velocity(track_idx, note_idx, new_velocity);
                                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                                self.history.push(crate::history::HistoryAction::ModifyNote {
                                                    track_idx,
                                                    note_idx,
                                                    old_note,
                                                    new_note: note.clone(),
                                                });
                                            }
                                        }
                                    }
                                    EditMode::Length => {
                                        if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                            let old_note = note.clone();
                                            let new_length = note.length.saturating_sub(self.sequencer.ticks_per_beat() / 4).max(1);
                                            self.sequencer.edit_note_length(track_idx, note_idx, new_length);
                                            if let Some(note) = self.sequencer.get_note(track_idx, note_idx) {
                                                self.history.push(crate::history::HistoryAction::ModifyNote {
                                                    track_idx,
                                                    note_idx,
                                                    old_note,
                                                    new_note: note.clone(),
                                                });
                                            }
                                        }
                                    }
                                    EditMode::None => {}
                                }
                            }
                            _ => {
                                self.handle_key(key)?;
                            }
                        }
                    } else {
                        self.handle_key(key)?;
                    }
                } else {
                    self.handle_key(key)?;
                }
            }
            Ok(())
        }
    }
}

