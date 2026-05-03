use cpal;
use cpal::SampleRate;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::signal::{self, Signal};
use dasp::slice::ToFrameSliceMut;
//added a few other stuff

//lets create some audio waves

//SIN variant other file is cos
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //get audio dev on sys for wav etc
    let wav_spec = hound::WavSpec {
        channels: 1, //
        sample_rate: 48000, //cd qual
        bits_per_sample: 16, //i16
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sampsin.wav", wav_spec).unwrap();
    //init CPAL
    let host = cpal::default_host();
    //could use devices
    let device = host.default_output_device().expect("no output device available");

    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets").unwrap();
    //conv to signal
    let total_samples = 5 * wav_spec.sample_rate;
    for i in 0..total_samples {
        let t = i as f32 / wav_spec.sample_rate as f32;
        let sample =  (i16::MAX as f32 * 0.5 * (std::f32::consts::TAU * 440.0 * t).sin()) as i16;
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
    let reader = hound::WavReader::open(("sampsin.wav")).unwrap();

    let spec = reader.spec();
    let samples = reader.into_samples::<i16>().filter_map(Result::ok);
    let mut frames = signal::from_interleaved_samples_iter::<_, [i16; 1]>(samples)
        .until_exhausted();

    //if wasnt iter based then have vecs to become one? like vec or slice into iter i think
    //set configs
    let config = cpal::StreamConfig {
        channels: spec.channels,
        sample_rate: (spec.sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };
    //dk what this line does
    let (complete_tx, complete_rx) = std::sync::mpsc::sync_channel(1);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let data_fn = move |data: &mut [i16], _info: &cpal::OutputCallbackInfo| {
        let buffer: &mut [[i16; 1]] = data.to_frame_slice_mut().unwrap();
        for out_frame in buffer {
            match frames.next() {
                Some(frame) => *out_frame = frame,
                None => {
                    // complete_tx.try_send(()).unwrap();
                    complete_tx.try_send(()).ok();
                    *out_frame = dasp::Frame::EQUILIBRIUM;
                }
            }
        }
    };
    let stream = device.build_output_stream(&config, data_fn, err_fn, None)?;
    stream.play().unwrap();

    //block until playback completes
    complete_rx.recv().unwrap();
    stream.pause().unwrap();
    Ok(())
}