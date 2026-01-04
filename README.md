# 🎹 Resonator

**Resonator** is a high-performance, real-time MIDI sequencer and mixer designed for the terminal. Built with **Rust**, it provides a distraction-free, low-latency environment for composing and routing MIDI signals through a professional Terminal User Interface (TUI) with rich colors, menus, and advanced features.

---

## 🚀 Overview

Resonator bridges the gap between the efficiency of the command line and the visual depth of a modern Digital Audio Workstation (DAW). By leveraging the `ratatui` ecosystem and Rust's systems-level precision, Resonator allows you to manage complex arrangements with microsecond timing and minimal CPU overhead.

### Why Resonator?

* **Performance:** Zero garbage collection pauses and ultra-low jitter.
* **Workflow:** Full keyboard-driven navigation (Vim-style) for rapid-fire composition.
* **Portability:** Run your entire studio over an SSH session.
* **Professional UI:** Rich color schemes, menus, visual feedback, and responsive layouts that adapt to any terminal size.

---

## ✨ Features

### Core Functionality
* **Multitrack Piano Roll:** Edit notes, velocities, and lengths with a high-resolution terminal grid featuring visual note rendering and color-coded tracks.
* **Step Sequencer:** Grid-based step sequencer for drum programming and pattern creation with visual step indicators and real-time playback.
* **Real-time Mixer:** Per-track volume controls with visual faders, panning, mute, and solo functionality.
* **Transport Control:** Global BPM, time signatures, play/pause/loop functionality with visual status indicators.
* **Device Routing:** Seamlessly connect to external hardware synths or software instruments via `midir`.
* **Live Preview:** Visualize your MIDI flow with a real-time track timeline, playhead, and current tick indicators.

### Professional UI Features
* **Rich Color Scheme:** Color-coded tracks, notes, and UI elements for intuitive navigation.
* **Theme System:** Multiple visual themes including retro styles:
  - **Retro Themes:** Retro (green terminal), Amber (amber monochrome), Matrix (green matrix), C64 (Commodore 64 blue), Apple2 (Apple II green/yellow), Monochrome (black & white), Neon (bright cyan/magenta)
  - **Standard Themes:** Default, Dark, Light, HighContrast
  - Press `` ` `` (backtick) or `F5` to cycle through themes
  - Theme selection is saved automatically
* **Menu System:** Accessible menu bar with File, Edit, Track, View, and Help menus.
* **Status Messages:** Real-time status feedback with auto-dismissing messages.
* **Help Overlay:** Comprehensive keyboard shortcuts reference (press `?`).
* **Responsive Layouts:** Automatically adapts to terminal size (compact mode for small terminals).
* **Visual Piano Roll:** 
  - Piano keyboard with note names (C, C#, D, etc.)
  - Beat markers and grid lines
  - Color-coded notes by track
  - Playhead and current tick visualization
  - Note selection highlighting
* **Enhanced Mixer:**
  - Visual volume faders with color-coded levels
  - Pan indicators (Left/Center/Right)
  - Mute and Solo columns
  - Track highlighting
* **Step Sequencer View:**
  - Grid-based step interface
  - Visual step indicators (active/inactive)
  - Current step highlighting during playback
  - Multiple tracks support
  - Per-step velocity and pitch control

### Advanced Features
* **Undo/Redo System:** Full history tracking with `Ctrl+Z` (undo) and `Ctrl+Y` (redo) supporting all editing operations.
* **Copy/Paste/Cut:** Duplicate and move notes with `Ctrl+C`, `Ctrl+V`, and `Ctrl+X`. Maintains relative timing when pasting.
* **Note Editing:** Direct editing of note properties:
  - Press `F2` to enter edit mode on selected note
  - `P` for pitch, `W` for velocity, `L` for length
  - Arrow keys (↑/↓) to adjust values
  - `Enter` to confirm, `Esc` to cancel
* **MIDI Filters:** Process MIDI data with filter chains:
  - Velocity scale and limit
  - Pitch shift and limit
  - Transpose
  - Quantize
* **Settings Management:** Comprehensive configuration system:
  - Persistent settings in JSON format (`resonator.json`)
  - Configurable defaults (BPM, velocity, note length)
  - Theme support with 11 themes (7 retro + 4 standard)
  - Auto-save configuration
  - Step sequencer configuration
* **Zoom Controls:** Adjustable zoom level for detailed editing (`+`/`-`).
* **Scrollable Views:** Navigate through your composition with `[` and `]` keys.
* **Track Management:** Add, delete, and rename tracks on the fly.
* **Note Selection:** Select and edit individual notes.
* **Project Management:** Save and load projects in JSON format (`.res` files).
* **Multiple Resolution Support:** Works seamlessly across different terminal sizes.

---

## 🛠 Installation

### Prerequisites

Ensure you have the Rust toolchain installed. Depending on your OS, you may need MIDI development headers:

* **Linux:** `libasound2-dev` (ALSA)
* **macOS:** CoreAudio (built-in)
* **Windows:** WinMM (built-in)

### Build from Source

```bash
git clone https://github.com/makalin/resonator.git
cd resonator
cargo build --release
./target/release/resonator
```

---

## ⌨️ Keyboard Shortcuts

### Transport Controls
| Key | Action |
| --- | --- |
| `Space` | Play / Stop |
| `R` | Reset to start |

### Navigation
| Key | Action |
| --- | --- |
| `H` / `←` | Move playhead left |
| `L` / `→` | Move playhead right |
| `J` / `↓` | Next track |
| `K` / `↑` | Previous track |
| `[` | Scroll view left |
| `]` | Scroll view right |

### Editing
| Key | Action |
| --- | --- |
| `Ins` / `Enter` | Add note at playhead |
| `Del` / `Backspace` | Delete selected note |
| `F2` | Enter note edit mode |
| `P` | Edit pitch (in edit mode) |
| `W` | Edit velocity (in edit mode) |
| `L` | Edit length (in edit mode) |
| `↑` / `↓` | Adjust value (in edit mode) |
| `+` / `=` | Zoom in |
| `-` / `_` | Zoom out |

### Views
| Key | Action |
| --- | --- |
| `1` | Piano Roll view |
| `2` | Mixer view |
| `3` | Settings view |
| `4` | Step Sequencer view |

### Menus
| Key | Action |
| --- | --- |
| `F` | File menu |
| `E` | Edit menu |
| `T` | Track menu |
| `V` | View menu |
| `H` | Help menu |
| `Esc` | Close menu / Cancel |

### File Operations
| Key | Action |
| --- | --- |
| `Ctrl + S` | Save project |
| `Ctrl + O` | Open project |
| `Ctrl + N` | New project |

### Edit Operations
| Key | Action |
| --- | --- |
| `Ctrl + Z` | Undo |
| `Ctrl + Y` | Redo |
| `Ctrl + C` | Copy selected notes |
| `Ctrl + V` | Paste notes at playhead |
| `Ctrl + X` | Cut selected notes |
| `Ctrl + A` | Select all (placeholder) |

### Track Management
| Key | Action |
| --- | --- |
| `T` | Add new track |
| `Ctrl + T` | Delete selected track |
| `M` | Toggle mute track |
| `S` | Toggle solo track (or toggle step in step sequencer) |

### Step Sequencer
| Key | Action |
| --- | --- |
| `S` | Toggle step at current position |
| `↑` / `↓` | Navigate between tracks |
| `←` / `→` | Navigate between steps |

### Theme Switching
| Key | Action |
| --- | --- |
| `` ` `` (backtick) | Cycle through themes (macOS-friendly) |
| `F5` | Cycle through themes (may require Fn key on macOS) |
| `Ctrl+Shift+T` | Cycle through themes (alternative) |

### System
| Key | Action |
| --- | --- |
| `Q` | Quit |
| `?` | Toggle help overlay |

---

## 🎨 UI Overview

### Piano Roll View
The main editing interface featuring:
- **Track List:** Left sidebar showing all tracks with selection indicators
- **Piano Grid:** Visual representation of notes on a piano keyboard
- **Note Visualization:** Color-coded notes showing pitch, timing, and track assignment
- **Playhead:** Red line indicating current playback position
- **Current Tick:** Green line showing real-time playback position
- **Note Selection:** Click or navigate to select notes for editing

### Step Sequencer View
Grid-based pattern sequencer for drum programming:
- **Step Grid:** Visual grid showing active/inactive steps
- **Track Rows:** Each track displayed as a row
- **Current Step Indicator:** Highlights the step currently playing
- **Step Toggle:** Activate/deactivate steps with `S` key
- **Visual Feedback:** Color-coded steps (active/inactive, current position)

### Mixer View
Professional mixing interface with:
- **Track Table:** Overview of all tracks with volume, pan, mute, and solo status
- **Volume Faders:** Visual representation of track volume levels
- **Color Coding:** Green (safe), Yellow (warning), Red (clipping) volume indicators

### Settings View
Project information and configuration:
- Track count and sequencer settings
- Zoom level and view controls
- Ticks per beat configuration
- Default BPM and project settings
- Current theme display
- History status (undo/redo availability)
- Clipboard status
- Settings can be saved to `resonator.json`

---

## 📝 Note Editing

Resonator provides comprehensive note editing capabilities:

1. **Select a Note:** Navigate to a note in the piano roll view
2. **Enter Edit Mode:** Press `F2` to start editing
3. **Choose Property:** 
   - `P` for pitch
   - `W` for velocity  
   - `L` for length
4. **Adjust Value:** Use `↑` to increase, `↓` to decrease
5. **Confirm:** Press `Enter` to save changes
6. **Cancel:** Press `Esc` to discard changes

All edits are tracked in history and can be undone with `Ctrl+Z`.

---

## 📋 Copy/Paste Workflow

1. **Copy Notes:**
   - Select notes in the piano roll
   - Press `Ctrl+C` to copy
   - Notes are stored in clipboard with relative timing

2. **Paste Notes:**
   - Move playhead to desired position
   - Press `Ctrl+V` to paste
   - Notes maintain relative timing from original position

3. **Cut Notes:**
   - Select notes
   - Press `Ctrl+X` to cut (copies and deletes)
   - Paste with `Ctrl+V`

---

## 🔄 Undo/Redo System

Resonator maintains a complete history of all editing operations:

- **Undo:** `Ctrl+Z` - Reverses the last operation
- **Redo:** `Ctrl+Y` - Re-applies a previously undone operation
- **History Tracking:** Supports up to 100 operations
- **Tracked Operations:**
  - Add/delete/modify notes
  - Add/delete tracks
  - Track renaming
  - Playhead movement

The history status is visible in the Settings view.

---

## 🎛️ MIDI Filters

Apply real-time MIDI processing with filter chains:

- **Velocity Scale:** Multiply velocity by a factor
- **Velocity Limit:** Clamp velocity to min/max range
- **Pitch Shift:** Transpose notes by semitones
- **Pitch Limit:** Clamp pitch to min/max range
- **Transpose:** Global pitch transposition
- **Quantize:** Snap notes to grid (applied during placement)

Filters are applied to MIDI output in real-time.

---

## ⚙️ Settings

Resonator saves configuration to `resonator.json` in the current directory:

**Configurable Settings:**
- Default BPM
- Default time signature
- Default velocity
- Default note length
- Auto-save interval
- Theme selection
- Piano roll zoom level
- Step sequencer steps and steps per beat
- MIDI output port selection

Settings are automatically loaded on startup and can be modified through the Settings view.

---

## 🎨 Themes

Resonator includes 11 visual themes to suit your preferences:

### Retro Themes
* **Retro** - Classic green-on-black terminal aesthetic
* **Amber** - Warm amber monochrome monitor style
* **Matrix** - Green matrix-style display
* **C64** - Commodore 64 blue color scheme
* **Apple2** - Apple II green and yellow palette
* **Monochrome** - Pure black and white minimalism
* **Neon** - Bright cyan and magenta neon colors

### Standard Themes
* **Default** - Balanced color scheme
* **Dark** - Dark mode with high contrast
* **Light** - Light mode for bright environments
* **HighContrast** - Maximum contrast for accessibility

**Switching Themes:**
- Press `` ` `` (backtick key, top-left above Tab) - Works well on macOS
- Press `F5` - May require Fn key on some keyboards
- Press `Ctrl+Shift+T` - Alternative combination

The current theme is displayed in the status bar and settings view. Theme selection is automatically saved and persists between sessions.

---

## 📁 Project Format

Projects are saved in JSON format with the `.res` extension. The format includes:
- All tracks and their notes (pitch, start time, length, velocity)
- Mixer settings (volume, pan, mute, solo per track)
- Transport settings (BPM, time signature, loop points)
- Playhead position

---

## 🗺 Roadmap

* [x] **Step Sequencer Mode:** A dedicated view for drum programming. ✅
* [x] **Undo/Redo System:** Full history support for editing operations. ✅
* [x] **Copy/Paste:** Duplicate notes and patterns. ✅
* [x] **Note Editing:** Direct pitch, velocity, and length editing. ✅
* [x] **Settings Management:** Comprehensive configuration system. ✅
* [x] **MIDI Filters:** Real-time MIDI processing. ✅
* [ ] **VST/AU Bridge:** Control plugin parameters directly via TUI.
* [ ] **MIDI Learn:** Quickly map physical controller knobs to TUI faders.
* [ ] **Scripting:** Lua support for generative MIDI patterns.
* [ ] **MIDI Input:** Record MIDI from external controllers.
* [ ] **Pattern Library:** Save and load step sequencer patterns.
* [ ] **Export:** Export to MIDI file format.

---

## 🐛 Known Issues

* MIDI output connection may fail silently if no MIDI devices are available (app continues to work without MIDI output).
* Some menu items are placeholders for future implementation.
* Step sequencer pattern export to piano roll is not yet implemented.

---

## 🤝 Contributing

Contributions are what make the open-source community an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 👤 Author

**Mehmet T. AKALIN**

  * **Digital Vision:** [dv.com.tr](https://dv.com.tr)
  * **GitHub:** [@makalin](https://github.com/makalin)
  * **X:** [@makalin](https://x.com/makalin)

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
