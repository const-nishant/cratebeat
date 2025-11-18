# 📘 Contributing to CrateBeat

Thanks for your interest in contributing!
CrateBeat is a small Rust-based beat-making tool — and contributions of all kinds are welcome.

This guide explains how to propose changes, file issues, and contribute code responsibly.

---

# 🛠 How to Contribute

## 1️⃣ Fork the Repository

Click **Fork** on GitHub and clone your fork locally:

```bash
git clone https://github.com/const-nishant/cratebeat
cd cratebeat
```

---

## 2️⃣ Create a Feature Branch

Use a descriptive branch name:

```
feat/add-swing
fix/bpm-timing-drift
refactor/audio-loader
docs/improve-readme
```

---

## 3️⃣ Follow Conventional Commits

Every commit **must** follow this style:

```
feat: add hats to step sequencer
fix: correct rodio device panic
chore: update dependencies
refactor: simplify UI rendering
docs: add contributing guide
```

This keeps the changelog clean and allows automated releases.

---

# 🧹 Code Style Guidelines

### ✔ Format before committing

```bash
cargo fmt
```

### ✔ Run the linter

```bash
cargo clippy -- -D warnings
```

### ✔ Keep changes small

Small, focused PRs are easier to review.

### ✔ Add comments if logic is complex

Clear code > clever code.

### ✔ No unsafe Rust unless absolutely necessary.

---

# 🧪 Testing Your Changes

Before submitting a PR, run:

```bash
cargo check
cargo test
cargo run
```

Test on your platform:

- Windows
- macOS
- Linux

If relevant, try at least two.

---

# 🧱 Project Structure Overview

```
src/
  app.rs          → App orchestrator / event loop
  config.rs       → App configuration
  ui/             → Terminal UI system
  audio/          → Sample loading, playback engine
  sequencer/      → BPM clock, playback, patterns
sounds/           → Local audio samples (ignored by git)
```

---

# 🐛 Reporting Issues

When opening an issue, include:

- What you expected to happen
- What actually happened
- Steps to reproduce
- Terminal output (if any)
- OS + Rust version (`rustc -V`)

Detailed issues get fixed faster.

---

# 💡 Suggesting Features

Open a **Discussion** or **Feature Request Issue** if you have ideas like:

- Effects (delay, lo-fi, distortion)
- 16/32-step sequencing
- MIDI input/output
- Pattern saving
- Custom themes / UI skins

Bold ideas welcome.

---

# 🔄 Submitting a Pull Request

1. Push your feature branch
2. Open a **PR to `main`**
3. Fill in the pull request template
4. Wait for CI to pass
5. A maintainer will review your changes

If requested, update the PR — it's normal!

---

# 📜 Licensing

By contributing to this repository, you agree your contributions will be licensed under the **MIT License**.

---

# 🎶 Thank You!

Your contributions help CrateBeat grow — whether it’s code, docs, ideas, or bug reports.
Feel free to reach out or open a discussion anytime.
