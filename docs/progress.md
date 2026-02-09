# Project Progress

## Current Status: Sine Wave Working, Starting Modularization

### Completed
- [x] Initialized Rust project with `cargo init`
- [x] Added `cpal` dependency for audio I/O
- [x] Created basic main.rs that connects to system audio device
- [x] **Generated a 440Hz sine wave through speakers**
- [x] Learned waveform math (sine, saw, square, triangle)

### Rust Concepts Learned
- `Option<T>` - Rust's null alternative (`Some(value)` or `None`)
- `unwrap()` / `expect()` - extracting values from Option/Result
- `match` and `if let` - pattern matching for handling Option
- `mut` - making variables mutable
- Type inference - Rust figures out types from context
- `target/` directory - where compiled code lives
- `f32` - 32-bit float, used for audio samples
- `move` closures - callback functions that own their variables
- `&mut [f32]` - mutable slice for writing audio samples
- `enum` - defining types with variants (like Waveform)
- `struct` and `impl` - defining data structures with methods
- `pub` - making items public across modules
- Rust module system - `mod` and `use`

### Audio/Synth Concepts Learned
- Digital audio = stream of samples (-1.0 to 1.0) at sample rate (44100Hz)
- Sine wave: `sin(2 * PI * phase)` - pure tone
- Saw wave: `2.0 * phase - 1.0` - buzzy, all harmonics
- Square wave: `if phase < 0.5 { 1.0 } else { -1.0 }` - hollow, odd harmonics
- Triangle wave: `4.0 * (phase - 0.5).abs() - 1.0` - soft, mellow
- Phase accumulator pattern for oscillators

---

## Next Session: Complete Modularization

### In Progress
Creating modular file structure:
```
src/
├── main.rs         # Entry point
├── audio.rs        # Audio device setup (TO CREATE)
├── oscillator.rs   # Waveforms (PARTIALLY DONE - code provided below)
```

### Code to Add to src/oscillator.rs
```rust
use std::f32::consts::PI;

pub enum Waveform {
    Sine,
    Saw,
    Square,
    Triangle,
}

pub struct Oscillator {
    pub waveform: Waveform,
    pub frequency: f32,
    pub phase: f32,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f32, sample_rate: f32) -> Self {
        Self {
            waveform,
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => (2.0 * PI * self.phase).sin(),
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Square => if self.phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Triangle => 4.0 * (self.phase - 0.5).abs() - 1.0,
        };

        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}
```

### Next Steps
1. Create `src/oscillator.rs` with the code above
2. Create `src/audio.rs` for device/stream setup
3. Update `main.rs` to use the modules
4. Test all 4 waveforms

---

## Project Goals (Long-term)
- [x] Basic oscillator (sine wave)
- [ ] Multiple waveforms (saw, square, triangle)
- [ ] ADSR envelope
- [ ] Filters (low-pass, high-pass)
- [ ] MIDI input support
- [ ] Sample loading and playback
- [ ] AI sample manipulation
- [ ] Text-to-sample generation
- [ ] Modern GUI
