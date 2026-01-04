# 🎹 Resonator

**Resonator** is a high-performance, real-time MIDI sequencer and mixer designed for the terminal. Built with **Rust**, it provides a distraction-free, low-latency environment for composing and routing MIDI signals through a professional Terminal User Interface (TUI).

---

## 🚀 Overview

Resonator bridges the gap between the efficiency of the command line and the visual depth of a modern Digital Audio Workstation (DAW). By leveraging the `ratatui` ecosystem and Rust's systems-level precision, Resonator allows you to manage complex arrangements with microsecond timing and minimal CPU overhead.

### Why Resonator?

* **Performance:** Zero garbage collection pauses and ultra-low jitter.
* **Workflow:** Full keyboard-driven navigation (Vim-style) for rapid-fire composition.
* **Portability:** Run your entire studio over an SSH session.

---

## ✨ Features

* **Multitrack Piano Roll:** Edit notes, velocities, and lengths with a high-resolution terminal grid.
* **Real-time Mixer:** Per-track volume controls, panning, and MIDI CC automation.
* **Transport Control:** Global BPM, time signatures, and play/pause/loop functionality.
* **Device Routing:** Seamlessly connect to external hardware synths or software instruments via `midir`.
* **Live Preview:** Visualize your MIDI flow with a real-time track timeline and keyboard overlay.

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

## ⌨️ Controls

| Category | Key | Action |
| --- | --- | --- |
| **Transport** | `Space` | Play / Stop |
| **Navigation** | `H/J/K/L` | Move Playhead / Move Through Tracks |
| **Editing** | `Ins` / `Enter` | Add Note at Playhead |
| **Editing** | `Backspace` | Delete Selected Note |
| **View** | `1` - `4` | Switch between Piano Roll, Mixer, and Settings |
| **System** | `Ctrl + S` | Save Project (`.res` format) |
| **System** | `Q` | Quit |

---

## 🗺 Roadmap

* [ ] **VST/AU Bridge:** Control plugin parameters directly via TUI.
* [ ] **Step Sequencer Mode:** A dedicated view for drum programming.
* [ ] **MIDI Learn:** Quickly map physical controller knobs to TUI faders.
* [ ] **Scripting:** Lua support for generative MIDI patterns.

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
