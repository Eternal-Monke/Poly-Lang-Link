# Poly-Lang-Link (PLL)

**Poly-Lang-Link (PLL)** is a multimedia processing tool that extracts, translates, and synchronizes anime dialogue across languages. It is built to support real-time workflows using modern audio and video processing libraries.

![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=black)

---

## ✨ Features

- 🎞️ Extracts audio from video files (e.g. `.mp4`, `.mkv`)
- 🔊 Splits audio into segments and detects blank or silent parts
- 🌍 Translates spoken dialogue using Whisper
- 👄 (Planned) Syncs translated dialogue with timestamps
- 📁 (Planned) Outputs translated subtitles and/or dubbed audio

---

## ⚙️ Requirements

- **Rust** (latest stable)
- **FFmpeg** (must be installed and in your `$PATH`)
- **Whisper-rs** (latest stable for translation)
- **hound** (latest stable for audio processing)

---

## 💻 Usage

```bash
cargo run -- <input_video> <output_wav>
```

## 🤝 Contributing

Want to help build the file sharing engine or GUI?
Feel free to fork, open an issue, or make a pull request!

## 📜 License

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html) © 2025 [Eternal-Monke](ht
