use crate::mixer::Mixer;
use crate::sequencer::Sequencer;
use crate::theme::ThemeColors;
use crate::transport::Transport;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Row, Table},
    Frame,
};

pub enum View {
    PianoRoll,
    Mixer,
    Settings,
    StepSequencer,
}

pub enum MenuState {
    None,
    File,
    Edit,
    Track,
    View,
    Help,
}

pub struct Tui {
    current_view: View,
    menu_state: MenuState,
    status_message: Option<String>,
    status_timer: u32,
    show_help: bool,
    show_quit_confirm: bool,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            current_view: View::PianoRoll,
            menu_state: MenuState::None,
            status_message: None,
            status_timer: 0,
            show_help: false,
            show_quit_confirm: false,
        }
    }

    pub fn set_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn current_view(&self) -> &View {
        &self.current_view
    }

    pub fn set_menu_state(&mut self, state: MenuState) {
        self.menu_state = state;
    }

    pub fn menu_state(&self) -> &MenuState {
        &self.menu_state
    }

    pub fn show_status(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_timer = 180; // 3 seconds at 60fps
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn show_quit_confirm(&self) -> bool {
        self.show_quit_confirm
    }

    pub fn set_quit_confirm(&mut self, show: bool) {
        self.show_quit_confirm = show;
    }

    fn theme_name(&self, theme: &crate::settings::Theme) -> &str {
        match theme {
            crate::settings::Theme::Default => "Default",
            crate::settings::Theme::Dark => "Dark",
            crate::settings::Theme::Light => "Light",
            crate::settings::Theme::HighContrast => "High Contrast",
            crate::settings::Theme::Retro => "Retro",
            crate::settings::Theme::Amber => "Amber",
            crate::settings::Theme::Matrix => "Matrix",
            crate::settings::Theme::C64 => "C64",
            crate::settings::Theme::Apple2 => "Apple II",
            crate::settings::Theme::Monochrome => "Monochrome",
            crate::settings::Theme::Neon => "Neon",
        }
    }

    pub fn tick(&mut self) {
        if self.status_timer > 0 {
            self.status_timer -= 1;
            if self.status_timer == 0 {
                self.status_message = None;
            }
        }
    }

    pub fn render(
        &self,
        f: &mut Frame,
        sequencer: &Sequencer,
        step_sequencer: &crate::step_sequencer::StepSequencer,
        mixer: &Mixer,
        transport: &Transport,
        settings: &crate::settings::Settings,
        history: &crate::history::History,
        clipboard: &crate::clipboard::Clipboard,
    ) {
        let size = f.size();
        
        // Responsive layout based on terminal size
        let chunks = if size.height < 30 {
            // Small terminal - compact layout
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Menu bar
                    Constraint::Length(2), // Transport
                    Constraint::Min(0),    // Main content
                    Constraint::Length(1), // Status
                ])
                .split(size)
        } else {
            // Normal terminal - full layout
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Menu bar
                    Constraint::Length(3), // Transport
                    Constraint::Min(0),    // Main content
                    Constraint::Length(2), // Status/Help
                ])
                .split(size)
        };

        let theme = ThemeColors::from_theme(&settings.theme);
        self.render_menu_bar(f, chunks[0], &theme);
        self.render_transport_bar(f, chunks[1], transport, sequencer, &theme);
        self.render_main_content(f, chunks[2], sequencer, step_sequencer, mixer, size, settings, history, clipboard, &theme);
        self.render_status_bar(f, chunks[chunks.len() - 1], &theme, settings);

        if self.show_help {
            self.render_help_overlay(f, size);
        }

        if self.show_quit_confirm {
            self.render_quit_confirm(f, size);
        }

        if !matches!(self.menu_state, MenuState::None) {
            self.render_menu(f, size);
        }
    }

    fn render_main_content(
        &self,
        f: &mut Frame,
        area: Rect,
        sequencer: &Sequencer,
        step_sequencer: &crate::step_sequencer::StepSequencer,
        mixer: &Mixer,
        size: Rect,
        settings: &crate::settings::Settings,
        history: &crate::history::History,
        clipboard: &crate::clipboard::Clipboard,
        theme: &ThemeColors,
    ) {
        match self.current_view {
            View::PianoRoll => self.render_piano_roll(f, area, sequencer, size, theme),
            View::Mixer => self.render_mixer(f, area, mixer, sequencer, theme),
            View::Settings => self.render_settings(f, area, sequencer, settings, history, clipboard, theme),
            View::StepSequencer => self.render_step_sequencer(f, area, step_sequencer, theme),
        }
    }

    fn render_menu_bar(&self, f: &mut Frame, area: Rect, theme: &ThemeColors) {
        let menu_items = vec![
            Span::styled(" [F]ile ", self.menu_style(MenuState::File, theme)),
            Span::styled(" [E]dit ", self.menu_style(MenuState::Edit, theme)),
            Span::styled(" [T]rack ", self.menu_style(MenuState::Track, theme)),
            Span::styled(" [V]iew ", self.menu_style(MenuState::View, theme)),
            Span::styled(" [H]elp ", self.menu_style(MenuState::Help, theme)),
        ];

        let line = Line::from(menu_items);
        let block = Block::default()
            .style(Style::default().bg(theme.menu_bg).fg(theme.menu_fg))
            .borders(Borders::NONE);
        let paragraph = Paragraph::new(line).block(block);
        f.render_widget(paragraph, area);
    }

    fn menu_style(&self, state: MenuState, theme: &ThemeColors) -> Style {
        if matches!((&self.menu_state, &state), (MenuState::File, MenuState::File) 
            | (MenuState::Edit, MenuState::Edit) 
            | (MenuState::Track, MenuState::Track)
            | (MenuState::View, MenuState::View)
            | (MenuState::Help, MenuState::Help)) {
            Style::default()
                .fg(theme.menu_selected)
                .bg(theme.menu_bg)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(theme.menu_fg).bg(theme.menu_bg)
        }
    }

    fn render_transport_bar(&self, f: &mut Frame, area: Rect, transport: &Transport, sequencer: &Sequencer, theme: &ThemeColors) {
        let status_icon = if transport.is_playing() {
            ("▶", theme.success, "PLAYING")
        } else {
            ("⏸", theme.error, "STOPPED")
        };

        let bpm_text = format!("BPM: {:.1}", transport.bpm());
        let time_sig = format!("{}/{}", transport.time_signature().0, transport.time_signature().1);
        let playhead = format!("Pos: {}", sequencer.playhead());
        let tick = format!("Tick: {}", sequencer.current_tick());

        let text = Line::from(vec![
            Span::styled(status_icon.0, Style::default().fg(status_icon.1).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(status_icon.2, Style::default().fg(status_icon.1)),
            Span::raw(" | "),
            Span::styled(&bpm_text, Style::default().fg(theme.info)),
            Span::raw(" | "),
            Span::styled(&time_sig, Style::default().fg(theme.info)),
            Span::raw(" | "),
            Span::styled(&playhead, Style::default().fg(theme.selected)),
            Span::raw(" | "),
            Span::styled(&tick, Style::default().fg(theme.info)),
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
            .title(" Transport ");
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    }


    fn render_piano_roll(
        &self,
        f: &mut Frame,
        area: Rect,
        sequencer: &Sequencer,
        size: Rect,
        theme: &ThemeColors,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(if size.width > 100 { 25 } else { 15 }),
                Constraint::Min(0),
            ])
            .split(area);

        self.render_track_list(f, chunks[0], sequencer, theme);
        self.render_piano_roll_grid(f, chunks[1], sequencer, theme);
    }

    fn render_track_list(&self, f: &mut Frame, area: Rect, sequencer: &Sequencer, theme: &ThemeColors) {
        let items: Vec<ListItem> = sequencer
            .tracks()
            .iter()
            .enumerate()
            .map(|(i, track)| {
                let is_selected = i == sequencer.selected_track();
                let style = if is_selected {
                    Style::default()
                        .fg(theme.selected)
                        .bg(theme.menu_bg)
                        .add_modifier(Modifier::BOLD)
                } else if track.muted {
                    Style::default().fg(theme.inactive)
                } else {
                    Style::default().fg(theme.foreground)
                };

                let mut text = format!(" {} ", track.name);
                if track.muted {
                    text.push_str("[M]");
                }
                if is_selected {
                    text.push_str(" ◄");
                }

                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border))
                    .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
                    .title(" Tracks ")
            );
        f.render_widget(list, area);
    }

    fn render_piano_roll_grid(&self, f: &mut Frame, area: Rect, sequencer: &Sequencer, theme: &ThemeColors) {
        // Piano roll visualization
        let visible_pitches = (area.height.saturating_sub(2)) as u8;
        let _visible_ticks = (area.width.saturating_sub(2)) as u32 * sequencer.zoom_level();
        
        let start_pitch = sequencer.view_start_pitch();
        let start_tick = sequencer.view_start_tick();
        let playhead = sequencer.playhead();
        let current_tick = sequencer.current_tick();

        // Create grid lines and notes
        let mut lines = Vec::new();
        
        // Draw piano keys on the left
        for y in 0..visible_pitches.min(88) {
            let pitch = start_pitch + (visible_pitches - 1 - y);
            if pitch > 127 {
                break;
            }
            
            let note_name = self.pitch_to_name(pitch);
            let is_black_key = matches!(note_name.chars().last(), Some('#'));

            let mut spans = vec![
                Span::styled(
                    format!("{:3} ", note_name),
                    Style::default()
                        .fg(if is_black_key { Color::DarkGray } else { Color::White })
                        .bg(if is_black_key { Color::Black } else { Color::DarkGray }),
                ),
            ];

            // Draw grid line and notes
            for x in 0..(area.width.saturating_sub(5)) {
                let tick = start_tick + (x as u32 * sequencer.zoom_level());
                
                // Draw playhead
                if tick == playhead || (tick < playhead && (tick + sequencer.zoom_level()) > playhead) {
                    spans.push(Span::styled("│", Style::default().fg(theme.playhead).add_modifier(Modifier::BOLD)));
                } else if tick == current_tick || (tick < current_tick && (tick + sequencer.zoom_level()) > current_tick) {
                    spans.push(Span::styled("│", Style::default().fg(theme.current_tick).add_modifier(Modifier::BOLD)));
                } else {
                    // Check for notes at this position
                    let mut has_note = false;
                    for (track_idx, track) in sequencer.tracks().iter().enumerate() {
                        for (note_idx, note) in track.notes.iter().enumerate() {
                            if note.pitch == pitch 
                                && note.start <= tick 
                                && (note.start + note.length) > tick {
                                let is_selected = sequencer.selected_note() == Some((track_idx, note_idx));
                                let color = if is_selected {
                                    theme.note_selected
                                } else if track_idx == sequencer.selected_track() {
                                    theme.note_color
                                } else {
                                    theme.inactive
                                };
                                spans.push(Span::styled("█", Style::default().fg(color)));
                                has_note = true;
                                break;
                            }
                        }
                        if has_note {
                            break;
                        }
                    }
                    if !has_note {
                        // Grid line
                        if tick % sequencer.ticks_per_beat() == 0 {
                            spans.push(Span::styled("│", Style::default().fg(theme.grid)));
                        } else if tick % (sequencer.ticks_per_beat() / 4) == 0 {
                            spans.push(Span::styled("·", Style::default().fg(theme.grid)));
                        } else {
                            spans.push(Span::raw(" "));
                        }
                    }
                }
            }

            lines.push(Line::from(spans));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
            .title(" Piano Roll ");
        
        let paragraph = Paragraph::new(lines).block(block);
        f.render_widget(paragraph, area);
    }

    fn pitch_to_name(&self, pitch: u8) -> String {
        let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (pitch / 12) - 1;
        let note = notes[(pitch % 12) as usize];
        format!("{}{}", note, octave)
    }

    fn render_mixer(&self, f: &mut Frame, area: Rect, mixer: &Mixer, sequencer: &Sequencer, theme: &ThemeColors) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(5)])
            .split(area);

        // Mixer table
        let rows: Vec<Row> = sequencer
            .tracks()
            .iter()
            .enumerate()
            .map(|(i, track)| {
                let mixer_track = mixer.get_track(i).unwrap_or(&crate::mixer::TrackMixer {
                    volume: 1.0,
                    pan: 0.0,
                    muted: false,
                    solo: false,
                });

                let pan_text = if mixer_track.pan.abs() < 0.01 {
                    "C".to_string()
                } else if mixer_track.pan > 0.0 {
                    format!("R{:.0}", mixer_track.pan * 100.0)
                } else {
                    format!("L{:.0}", mixer_track.pan.abs() * 100.0)
                };

                Row::new(vec![
                    format!("{:2} {}", i + 1, track.name),
                    format!("{:.0}%", mixer_track.volume * 100.0),
                    pan_text,
                    if mixer_track.muted { "MUTE" } else { "" }.to_string(),
                    if mixer_track.solo { "SOLO" } else { "" }.to_string(),
                ])
                .style(if i == sequencer.selected_track() {
                    Style::default().fg(theme.selected).bg(theme.menu_bg)
                } else {
                    Style::default()
                })
            })
            .collect();

        let widths = [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ];
        
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec!["Track", "Volume", "Pan", "Mute", "Solo"])
                    .style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border))
                    .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
                    .title(" Mixer ")
            );

        f.render_widget(table, chunks[0]);

        // Volume faders visualization
        if chunks[1].height > 3 {
            let fader_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(100 / sequencer.tracks().len().max(1) as u16); sequencer.tracks().len().max(1)])
                .split(chunks[1]);

            for (i, fader_area) in fader_chunks.iter().enumerate() {
                if i < sequencer.tracks().len() {
                    let mixer_track = mixer.get_track(i).unwrap_or(&crate::mixer::TrackMixer {
                        volume: 1.0,
                        pan: 0.0,
                        muted: false,
                        solo: false,
                    });

                    let volume_ratio = mixer_track.volume;
                    let gauge = Gauge::default()
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(format!("T{}", i + 1))
                        )
                        .gauge_style(
                            Style::default()
                                .fg(if volume_ratio > 0.9 {
                                    theme.error
                                } else if volume_ratio > 0.7 {
                                    theme.warning
                                } else {
                                    theme.success
                                })
                                .bg(theme.menu_bg)
                        )
                        .ratio(volume_ratio as f64)
                        .label(format!("{:.0}%", volume_ratio * 100.0));

                    f.render_widget(gauge, *fader_area);
                }
            }
        }
    }

    fn render_settings(&self, f: &mut Frame, area: Rect, sequencer: &Sequencer, settings: &crate::settings::Settings, history: &crate::history::History, clipboard: &crate::clipboard::Clipboard, theme: &ThemeColors) {
        let text = vec![
            Line::from(vec![
                Span::styled("Settings", Style::default().fg(theme.title).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tracks: ", Style::default().fg(theme.selected)),
                Span::raw(format!("{}", sequencer.tracks().len())),
            ]),
            Line::from(vec![
                Span::styled("Zoom: ", Style::default().fg(theme.selected)),
                Span::raw(format!("{}", sequencer.zoom_level())),
            ]),
            Line::from(vec![
                Span::styled("Ticks per Beat: ", Style::default().fg(theme.selected)),
                Span::raw(format!("{}", sequencer.ticks_per_beat())),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Default BPM: ", Style::default().fg(theme.selected)),
                Span::raw(format!("{:.1}", settings.default_bpm)),
            ]),
            Line::from(vec![
                Span::styled("Theme: ", Style::default().fg(theme.selected)),
                Span::raw(self.theme_name(&settings.theme)),
            ]),
            Line::from(vec![
                Span::styled("History: ", Style::default().fg(theme.selected)),
                Span::raw(format!("Undo: {}, Redo: {}", if history.can_undo() { "Yes" } else { "No" }, if history.can_redo() { "Yes" } else { "No" })),
            ]),
            Line::from(vec![
                Span::styled("Clipboard: ", Style::default().fg(theme.selected)),
                Span::raw(if clipboard.has_data() { "Has data" } else { "Empty" }),
            ]),
            Line::from(""),
            Line::from("Use +/- to adjust zoom, [ and ] to scroll"),
            Line::from("Press ` (backtick) or F5 to cycle themes"),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
            .title(" Settings ");
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    }

    fn render_status_bar(&self, f: &mut Frame, area: Rect, theme: &ThemeColors, settings: &crate::settings::Settings) {
        let status_text = if let Some(ref msg) = self.status_message {
            msg.clone()
        } else {
            "Ready".to_string()
        };

        let view_name = match self.current_view {
            View::PianoRoll => "Piano Roll",
            View::Mixer => "Mixer",
            View::Settings => "Settings",
            View::StepSequencer => "Step Sequencer",
        };

        let help_hint = format!("View: {} | Theme: {} | Press '`' (backtick) or F5 to cycle themes | '?' for help | Q to quit", view_name, self.theme_name(&settings.theme));
        let full_text = format!("{} | {}", status_text, help_hint);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .style(Style::default().bg(theme.background).fg(theme.foreground));
        
        let paragraph = Paragraph::new(full_text)
            .block(block)
            .alignment(Alignment::Left);
        f.render_widget(paragraph, area);
    }

    fn render_help_overlay(&self, f: &mut Frame, size: Rect) {
        let popup_area = self.centered_rect(65, 90, size);
        
        let help_text = vec![
            Line::from(vec![Span::styled("Keyboard Shortcuts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from(vec![Span::styled("Transport:", Style::default().fg(Color::Yellow))]),
            Line::from("  Space     - Play/Stop"),
            Line::from("  R         - Reset to start"),
            Line::from(""),
            Line::from(vec![Span::styled("Navigation:", Style::default().fg(Color::Yellow))]),
            Line::from("  H/←       - Move playhead left"),
            Line::from("  L/→       - Move playhead right"),
            Line::from("  J/↓       - Next track"),
            Line::from("  K/↑       - Previous track"),
            Line::from("  [/]       - Scroll view"),
            Line::from(""),
            Line::from(vec![Span::styled("Views:", Style::default().fg(Color::Yellow))]),
            Line::from("  1         - Piano Roll"),
            Line::from("  2         - Mixer"),
            Line::from("  3         - Settings"),
            Line::from("  4         - Step Sequencer"),
            Line::from(""),
            Line::from(vec![Span::styled("Editing:", Style::default().fg(Color::Yellow))]),
            Line::from("  Ins/Enter - Add note"),
            Line::from("  Del       - Delete note"),
            Line::from("  F2        - Edit selected note"),
            Line::from("  P/W/L     - Edit pitch/velocity/length"),
            Line::from("  ↑/↓       - Adjust value (in edit mode)"),
            Line::from("  +/-       - Adjust zoom"),
            Line::from(""),
            Line::from(vec![Span::styled("Undo/Redo:", Style::default().fg(Color::Yellow))]),
            Line::from("  Ctrl+Z    - Undo"),
            Line::from("  Ctrl+Y    - Redo"),
            Line::from(""),
            Line::from(vec![Span::styled("Copy/Paste:", Style::default().fg(Color::Yellow))]),
            Line::from("  Ctrl+C    - Copy notes"),
            Line::from("  Ctrl+V    - Paste notes"),
            Line::from("  Ctrl+X    - Cut notes"),
            Line::from(""),
            Line::from(vec![Span::styled("Step Sequencer:", Style::default().fg(Color::Yellow))]),
            Line::from("  S         - Toggle step"),
            Line::from("  ↑/↓       - Navigate tracks"),
            Line::from("  ←/→       - Navigate steps"),
            Line::from(""),
            Line::from(vec![Span::styled("Menus:", Style::default().fg(Color::Yellow))]),
            Line::from("  F         - File menu"),
            Line::from("  E         - Edit menu"),
            Line::from("  T         - Track menu"),
            Line::from("  V         - View menu"),
            Line::from("  ?         - Toggle help"),
            Line::from(""),
            Line::from(vec![Span::styled("File:", Style::default().fg(Color::Yellow))]),
            Line::from("  Ctrl+S    - Save project"),
            Line::from("  Ctrl+O    - Open project"),
            Line::from("  Ctrl+N    - New project"),
            Line::from(""),
            Line::from(vec![Span::styled("Track:", Style::default().fg(Color::Yellow))]),
            Line::from("  T         - Add track"),
            Line::from("  Ctrl+T    - Delete track"),
            Line::from("  M         - Mute track"),
            Line::from("  S         - Solo track"),
            Line::from(""),
            Line::from(vec![Span::styled("System:", Style::default().fg(Color::Yellow))]),
            Line::from("  Q         - Quit"),
            Line::from("  Esc       - Cancel/Close menu"),
        ];

        // Render background overlay
        let bg_block = Block::default()
            .style(Style::default().bg(Color::Black));
        f.render_widget(bg_block, size);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .title(" Help (Press ? to close) ")
            .style(Style::default().bg(Color::Black));

        let paragraph = Paragraph::new(help_text)
            .block(block)
            .alignment(Alignment::Left);
        f.render_widget(paragraph, popup_area);
    }

    fn render_menu(&self, f: &mut Frame, size: Rect) {
        let menu_items = match self.menu_state {
            MenuState::File => vec![
                "New Project (Ctrl+N)",
                "Open Project (Ctrl+O)",
                "Save Project (Ctrl+S)",
                "Save As...",
                "---",
                "Quit (Q)",
            ],
            MenuState::Edit => vec![
                "Undo (Ctrl+Z)",
                "Redo (Ctrl+Y)",
                "---",
                "Cut (Ctrl+X)",
                "Copy (Ctrl+C)",
                "Paste (Ctrl+V)",
                "---",
                "Select All (Ctrl+A)",
            ],
            MenuState::Track => vec![
                "Add Track (N)",
                "Delete Track (Ctrl+T)",
                "Rename Track (F2)",
                "---",
                "Mute Track (M)",
                "Solo Track (S)",
            ],
            MenuState::View => vec![
                "Piano Roll (1)",
                "Mixer (2)",
                "Settings (3)",
                "Step Sequencer (4)",
                "---",
                "Zoom In (+)",
                "Zoom Out (-)",
                "---",
                "Scroll Left ([)",
                "Scroll Right (])",
            ],
            MenuState::Help => vec![
                "Keyboard Shortcuts (?)",
                "About",
            ],
            MenuState::None => vec![],
        };

        if menu_items.is_empty() {
            return;
        }

        let popup_width = menu_items.iter().map(|s| s.len()).max().unwrap_or(20) + 4;
        // Calculate height: each item needs 1 line, plus 2 for borders/title, plus 2 for padding
        // For View menu with 10 items, we need at least 14 lines
        let item_count = menu_items.len();
        let min_lines = (item_count + 4).max(14); // Ensure at least 14 lines for View menu
        // Convert to percentage of screen height, but ensure it's visible
        let popup_height_percent = ((min_lines as f32 / size.height as f32) * 100.0).min(90.0).max(30.0) as u16;
        let popup_area = self.centered_rect(popup_width as u16, popup_height_percent, size);

        let items: Vec<ListItem> = menu_items
            .iter()
            .map(|item| {
                if item == &"---" {
                    ListItem::new("─".repeat(popup_width as usize - 2))
                        .style(Style::default().fg(Color::DarkGray))
                } else {
                    ListItem::new(format!(" {}", item))
                        .style(Style::default().fg(Color::White))
                }
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                    .title(match self.menu_state {
                        MenuState::File => " File ",
                        MenuState::Edit => " Edit ",
                        MenuState::Track => " Track ",
                        MenuState::View => " View ",
                        MenuState::Help => " Help ",
                        MenuState::None => "",
                    })
                    .style(Style::default().bg(Color::Black))
            );

        f.render_widget(list, popup_area);
        
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    fn render_step_sequencer(&self, f: &mut Frame, area: Rect, step_sequencer: &crate::step_sequencer::StepSequencer, theme: &ThemeColors) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        // Header
        let header_text = format!("Step Sequencer - {} steps, {} steps/beat", step_sequencer.num_steps(), step_sequencer.steps_per_beat());
        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title(" Step Sequencer "))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(header, chunks[0]);

        // Step grid
        let track_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3); step_sequencer.tracks().len()])
            .split(chunks[1]);

        for (i, track_chunk) in track_chunks.iter().enumerate() {
            if i < step_sequencer.tracks().len() {
                let track = &step_sequencer.tracks()[i];
                let step_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Length(3); step_sequencer.num_steps()])
                    .split(*track_chunk);

                let mut step_spans = vec![Span::styled(
                    format!("{:2} ", i + 1),
                    if i == step_sequencer.selected_track() {
                        Style::default().fg(theme.selected).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.foreground)
                    },
                )];

                for (j, step) in track.steps.iter().enumerate() {
                    let is_current = j == step_sequencer.current_step();
                    let style = if step.active {
                        if is_current {
                            Style::default().fg(theme.success).bg(theme.menu_bg).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(theme.active).bg(theme.menu_bg)
                        }
                    } else {
                        if is_current {
                            Style::default().fg(theme.inactive).bg(theme.background)
                        } else {
                            Style::default().fg(theme.inactive)
                        }
                    };
                    step_spans.push(Span::styled(if step.active { "●" } else { "○" }, style));
                }

                let line = Line::from(step_spans);
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border))
                    .title(track.name.as_str());
                let paragraph = Paragraph::new(line).block(block);
                f.render_widget(paragraph, *track_chunk);
            }
        }
    }

    fn render_quit_confirm(&self, f: &mut Frame, size: Rect) {
        let popup_area = self.centered_rect(50, 25, size);
        
        let confirm_text = vec![
            Line::from(vec![Span::styled("Quit Resonator?", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from("Are you sure you want to quit?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" - Yes, Quit"),
            ]),
            Line::from(vec![
                Span::styled("N", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" - Cancel"),
            ]),
        ];

        // Render background overlay
        let bg_block = Block::default()
            .style(Style::default().bg(Color::Black));
        f.render_widget(bg_block, size);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .title(" Quit Confirmation ")
            .style(Style::default().bg(Color::Black));

        let paragraph = Paragraph::new(confirm_text)
            .block(block)
            .alignment(Alignment::Left);
        f.render_widget(paragraph, popup_area);
    }
}

