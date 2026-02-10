# MIDI Note to Program Change

A lightweight MIDI utility plugin built in Rust using the [nih-plug](https://github.com/robbert-vdh/nih-plug) framework.

## Description

This plugin converts incoming MIDI notes into MIDI Program Change messages. It is designed for musicians and producers who want to trigger preset changes using a MIDI keyboard or pads without complex routing.

### Key Features
- **Strict Note-to-PC Mapping**:
    - **Base Note**: C0 (MIDI Note 24) maps to **Program Change 0**.
    - **Range**: Supports Program Changes from 0 to 99 (mapped from notes C0 to D#8).
- **Intelligent Filtering**:
    - Out-of-range notes are automatically blocked.
    - Note Off messages are ignored (to prevent double-triggering).
    - Other MIDI data (CC, Pitch Bend, etc.) is passed through untouched.
- **Minimalist Design**: No GUI or unnecessary parameters for maximum performance and low latency.
- **Audio Pass-through**: Declares stereo audio I/O for 100% compatibility with hosts like Ableton Live, Bitwig, and Reaper.

## Download

You can download the latest compiled versions here:

- **[Download VST3 / CLAP](https://github.com/nico7an/MidiNoteToProgramChange/releases)** 
*(Note: Replace this link with your actual GitHub Releases URL once the project is pushed)*

## Installation

### Windows
1. Copy the `midi_note_to_pc.vst3` folder to your VST3 directory (usually `C:\Program Files\Common Files\VST3`).
2. Rescan your plugins in your DAW.

## Build from Source

If you have the Rust toolchain installed:

1. Clone the repository.
2. Run the build script:
   ```powershell
   ./build.bat
   ```
3. The bundled artifacts will be in `target/bundled/`.

## Author
- **Nico7an** - [GitHub](https://github.com/nico7an)

## License
MIT
