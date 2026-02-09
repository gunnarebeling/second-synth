use std::f32::consts::PI;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config().unwrap();
    println!("Sample rate: {}", config.sample_rate().0);

    let sample_rate = config.sample_rate().0 as f32;
    let mut phase: f32 = 0.0;
    let frequency: f32 = 440.0;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = (2.0 * PI * phase).sin() * 0.5;
                    phase += frequency / sample_rate;
                    if phase >= 1.0 {
                        phase -= 1.0;
                    }
                }
            },
            |err| eprintln!("Audio error : {}", err),
            None,
        )
        .unwrap();
    stream.play().unwrap();

    println!("Playing 440hz sine wave. Press Enter to Stop.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
