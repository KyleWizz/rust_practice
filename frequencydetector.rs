use cpal::SampleRate;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::signal::{self, Signal};
use dasp::slice::ToFrameSliceMut;
use rustfft::{FftPlanner, num_complex::Complex};
use rustfft::num_traits::Zero;

fn main() {

    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets").unwrap();
    let reader = hound::WavReader::open("sampsin.wav").unwrap();
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let mut planner = FftPlanner::new();
    let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1024];
    let samples: Vec<i16> = reader.into_samples::<i16>().filter_map(Result::ok).collect();

    for s in 0..1024 {
        buffer[s].re = samples[s] as f32 / i16::MAX as f32;
    }
//perform forward fast fourier transform
    let fft = planner.plan_fft_forward(1024);
    //add a buffer for bytes
    //Creates the input/output buffer - a Vec of 1024 complex numbers,
    // all starting at zero. Each Complex has imaginary and real
    //let mut buffer = vec![Complex{re: 0.0f32, im: 0.0f32}; 1024];
    //process our buffer- writes the frequency data back into the same buffer
    //each slot will represent a frequency bin
    fft.process(&mut buffer);
    //freq hertz

    //mag = sqr re^2 + im^2
    for i in 0..buffer.len(){
        //we wanna get freq and magnitude here - div sample rate at i by buffer len for
        //44100 samples
        let frequency_hertz= i as f32 * sample_rate / buffer.len() as f32;
        let magnitude = ((buffer[i].re * buffer[i].re) + (buffer[i].im * buffer[i].im)).sqrt();
        println!("frequency: {} magnitude: {}", frequency_hertz, magnitude);
    }
}