# Rust Notes

Quick reference for Rust concepts learned while building this synth.

## Option - Rust's Null Alternative

```rust
enum Option<T> {
    Some(T),    // Has a value
    None,       // No value (like null, but explicit)
}
```

**Getting the value out:**
```rust
value.unwrap()              // Crash if None
value.expect("error msg")   // Crash with message if None
value.unwrap_or(default)    // Use default if None

// Pattern matching (safest)
match value {
    Some(v) => println!("{}", v),
    None => println!("nothing"),
}

if let Some(v) = value {
    println!("{}", v);
}
```

## Variables

```rust
let x = 5;          // Immutable (can't change)
let mut y = 5;      // Mutable (can change)
y = 10;             // OK

let z;              // Declared but uninitialized
z = 15;             // Must assign before use
```

## Type Inference

Rust infers types from values:
```rust
let x = 5;       // i32 (default integer)
let y = 5.0;     // f64 (default float)
let z = "hi";    // &str (string slice)
```

For audio, common types:
- `f32` - sample values (-1.0 to 1.0)
- `i32` / `usize` - indices
- `u8` - MIDI values (0-127)

## Structs and Impl

```rust
pub struct Oscillator {
    pub frequency: f32,    // pub = accessible from outside
    phase: f32,            // private by default
}

impl Oscillator {
    // Constructor (convention: called "new")
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
        }
    }

    // Method that modifies self
    pub fn next_sample(&mut self) -> f32 {
        // ...
    }
}
```

## Enums

```rust
pub enum Waveform {
    Sine,
    Saw,
    Square,
    Triangle,
}

// Use with match
match waveform {
    Waveform::Sine => /* ... */,
    Waveform::Saw => /* ... */,
    // ...
}
```

## Closures

Anonymous functions, used for callbacks:
```rust
// Basic closure
let add = |a, b| a + b;

// move closure - takes ownership of captured variables
let mut phase = 0.0;
let callback = move |data: &mut [f32]| {
    // phase is now owned by this closure
};
```

## Modules

```rust
// In main.rs - declare modules
mod oscillator;    // Loads src/oscillator.rs
mod audio;         // Loads src/audio.rs

// Import specific items
use oscillator::{Oscillator, Waveform};
```

## Build Commands

```bash
cargo run              # Build and run (debug)
cargo run --release    # Build and run (optimized)
cargo build            # Just build
cargo clean            # Delete target/
```
