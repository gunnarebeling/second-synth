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
- `f32` - sample values
- `i32` / `usize` - indices
- `u8` - MIDI values (0-127)

## Build Commands

```bash
cargo run              # Build and run (debug)
cargo run --release    # Build and run (optimized)
cargo build            # Just build
cargo clean            # Delete target/
```
