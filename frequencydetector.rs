use cpal::SampleRate;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::signal::{self, Signal};
use dasp::slice::ToFrameSliceMut;
use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
    let mut planner = FftPlanner::new();
    //perform forward fast fourier transform
    let fft = planner.plan_fft_forward(1024);
    //add a buffer for bytes
    //Creates the input/output buffer - a Vec of 1024 complex numbers,
    // all starting at zero. Each Complex has imaginary and real
    let mut buffer = vec![Complex{re: 0.0f32, im: 0.0f32}; 1024];
    //process our buffer- writes the frequency data back into the same buffer
    //each slot will represent a frequency bin
    fft.process(&mut buffer);
    let sample_rate = 44100.0f32;
    //mag = sqr re^2 + im^2
    for i in 0..buffer.len(){
        let frequency_hertz= i as f32 * sample_rate / buffer.len() as f32;
        let magnitude = ((buffer[i].re * buffer[i].re) + (buffer[i].im * buffer[i].im)).sqrt();
        println!("frequency: {} magnitude: {}", frequency_hertz, magnitude);
    }


}