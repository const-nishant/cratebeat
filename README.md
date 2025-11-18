# 🎧 **CrateBeat**

A tiny, terminal-based **beat maker** built in **Rust** — featuring real-time sample playback, a step sequencer, and a clean ASCII TUI.
Make beats using your keyboard, trigger sounds live, adjust BPM, and watch the sequencer animate inside your terminal.

```
  ____            _        ____           
 / ___|___  _ __ | |_ ___ / ___| ___  ___ 
| |   / _ \| '_ \| __/ _ \ |  _ / _ \/ __|
| |__| (_) | | | | ||  __/ |_| |  __/\__ \
 \____\___/|_| |_|\__\___|\____|\___||___/
                                          
      CrateBeat - Terminal Beat Maker
```

---

# 🚀 Features

### 🎛 **1. Step Sequencer (8 Steps)**

* 4 tracks: Kick, Snare, Hat, Clap
* Toggle steps visually in a grid
* Real-time playhead animation
* Adjustable BPM (20–300)

### 🎹 **2. Manual Drum Pads**

* `a` → Kick
* `s` → Snare
* `d` → Hat
* `f` → Clap
  Perfect for tapping out ideas live.

### 🎧 **3. Audio Engine (rodio)**

* Loads WAV samples from the `/sounds` folder
* Low-latency playback
* Multiple sounds can play together

### 🖥 **4. Interactive TUI (ratatui + crossterm)**

* Clean ASCII design
* Animations + banners
* Arrow-key navigation
* Works on any terminal

---

# 🗂 Project Structure

```
cratebeat/
│  Cargo.toml
│  README.md
│
├── sounds/
│   ├── kick.wav
│   ├── snare.wav
│   ├── hat.wav
│   └── clap.wav
│
└── src/
    ├── main.rs
    ├── app.rs
    ├── config.rs
    │
    ├── ui/
    │   ├── mod.rs
    │   ├── layout.rs
    │   ├── input.rs
    │   └── ascii.rs
    │
    ├── audio/
    │   ├── mod.rs
    │   ├── loader.rs
    │   ├── player.rs
    │   └── mixer.rs
    │
    └── sequencer/
        ├── mod.rs
        ├── clock.rs
        ├── steps.rs
        └── engine.rs
```

---

# 🎮 Controls

### **Global**

| Key | Action                |
| --- | --------------------- |
| `q` | Quit CrateBeat        |
| `p` | Play / Stop sequencer |
| `+` | Increase BPM by 5     |
| `-` | Decrease BPM by 5     |

### **Sequencer Grid**

| Key     | Action             |
| ------- | ------------------ |
| `←` `→` | Move step cursor   |
| `↑` `↓` | Move track cursor  |
| `Space` | Toggle step on/off |

### **Drum Pads**

| Key | Sound |
| --- | ----- |
| `a` | Kick  |
| `s` | Snare |
| `d` | Hat   |
| `f` | Clap  |

---

# 🔊 Adding Your Own Sounds

Add WAV files to `./sounds/`:

```
sounds/
  kick.wav
  snare.wav
  hat.wav
  clap.wav
```

You can replace them with any short percussion samples you like.

---

# 🛠 Installation (Windows, macOS, Linux)

### **1. Ensure you have Rust**

[https://rustup.rs](https://rustup.rs)

### **2. Clone & run**

```bash
git clone https://github.com/yourname/cratebeat
cd cratebeat
cargo run --release
```

### ⚠ Windows Only — Requires MSVC Build Tools

If you see a `link.exe` error:

You must install:

* **Visual Studio Build Tools**
* ✔ Desktop Development With C++
* ✔ Windows 10/11 SDK

---

# 🤝 Contributing

PRs welcome — especially:

* New sound packs
* TUI improvements
* Effects (delay, bitcrush, filters)
* Pattern saving/loading

---

# 📜 License

MIT — do whatever you want.

---

# ❤️ Made with Rust, ASCII Art, and a lot of rhythm.

