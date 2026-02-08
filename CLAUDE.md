# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **learning-focused** software synthesizer project built in Rust. The primary goals are:
1. Learn Rust through hands-on synth development
2. Understand how software synthesizers work internally
3. Build a modern UI for the synth
4. Integrate AI for sample manipulation and text-to-sample generation

**Important**: The user wants to write code manually. Claude should explain concepts, guide implementation, and teach rather than write complete solutions.

## Planned Architecture

### Core Components (to be built)
- **Audio Engine**: Real-time audio processing using `cpal` for cross-platform audio I/O
- **DSP Module**: Oscillators, filters, envelopes, effects
- **Sampler**: Load, manipulate, and spread samples across keyboard
- **MIDI Handler**: MIDI input processing for keyboard control
- **AI Integration**: Text-to-sample generation, sample manipulation
- **GUI**: Modern interface (likely using `iced` or `egui`)

### Key Rust Crates to Learn
- `cpal` - Cross-platform audio I/O
- `rodio` - Audio playback built on cpal
- `fundsp` - Audio DSP library
- `midir` - MIDI I/O
- `iced` or `egui` - GUI frameworks
- AI integration via API calls to audio generation services

## Build Commands

Once Cargo.toml is set up:
```bash
cargo build              # Build the project
cargo run                # Run the synth
cargo test               # Run tests
cargo test test_name     # Run a single test
cargo clippy             # Lint the code
cargo fmt                # Format code
```

## Teaching Approach

When helping with this project:
1. Explain the "why" behind synth concepts (oscillators, filters, ADSR, etc.)
2. Break down Rust concepts as they come up
3. Suggest what to implement next and explain the approach
4. Let the user write the code themselves
5. Review and explain improvements when asked
