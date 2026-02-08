use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device available");
    println!("Using output device: {}", device.name().unwrap());
}
