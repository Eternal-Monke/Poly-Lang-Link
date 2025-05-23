# Poly-Lang-Link (PLL)

**Poly-Lang-Link (PLL)** is a multimedia processing tool that extracts, translates, and synchronizes dialogue across languages. It is built to support batch or real-time workflows using modern audio and video processing libraries.

---

## ✨ Features

- 🎞️ Extracts audio from video files (e.g. `.mp4`, `.mkv`)
- 🔊 Splits audio into segments and detects blank or silent parts
- 🧠 (NOT COMPLETELY WORKING) Transcribes and translates spoken dialogue using Whisper
- 👄 (NOT YET IMPLEMENTED) Syncs translated dialogue with character lip movements
- 📁 (NOT YET IMPLEMENTED) Outputs translated subtitles

---

## ⚙️ Requirements

- **Rust** (latest stable)
- **FFmpeg** (must be installed and in your `$PATH`)
- **Whisper.cpp** https://huggingface.co/ggerganov/whisper.cpp/tree/main

---

## 🚀 Usage

```bash
cargo run -- <input_video> <output>
