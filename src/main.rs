//! A basic portaudio + pocketsphinx example.
//!
//! Audio from the default input device is passed to pocketsphinx for 5 seconds, we get the
//! recognized utterances, rince and repeat.

extern crate portaudio;
extern crate pocketsphinx;

use portaudio as pa;
use std::time::{ Duration, SystemTime };

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 256;
const CHANNELS: i32 = 1;
const INTERLEAVED: bool = true;

fn main() {
    run().unwrap()
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn run() -> Result<(), pa::Error> {

    let pa = try!(pa::PortAudio::new());

    println!("PortAudio:");
    println!("version: {}", pa.version());
    println!("version text: {:?}", pa.version_text());
    println!("host count: {}", try!(pa.host_api_count()));

    let default_host = try!(pa.default_host_api());
    println!("default host: {:#?}", pa.host_api_info(default_host));

    let def_input = try!(pa.default_input_device());
    let input_info = try!(pa.device_info(def_input));
    println!("Default input device info: {:#?}", &input_info);

    // Construct the input stream parameters.
    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<i16>::new(def_input, CHANNELS, INTERLEAVED, latency);

    // Construct the settings with which we'll open our input stream.
    let settings = pa::InputStreamSettings::new(input_params, SAMPLE_RATE, FRAMES);

    // We'll use this channel to send the samples to analyze.
    let (sender, receiver) = ::std::sync::mpsc::channel();

    // A callback to pass to the non-blocking stream.
    let callback = move |pa::InputStreamCallbackArgs { buffer, frames, flags, time }| {
        assert!(frames == FRAMES as usize);
        sender.send(buffer).ok();

        pa::Continue // pa::Complete to stop.
    };

    // Construct a stream with input and output sample types of i16.
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    // Loop while the non-blocking stream is active.
    let mut start = SystemTime::now();
    let delay = Duration::new(5, 0);

    let ps_config = pocketsphinx::CmdLn::init(true, &[
        "-hmm", "./model/en-us/en-us",
        "-lm", "./model/en-us/en-us.lm.bin",
        "-dict", "./model/en-us/cmudict-en-us.dict",
        ]).unwrap();
    let ps_decoder = pocketsphinx::PsDecoder::init(ps_config);
    println!("Before start_utt()");
    ps_decoder.start_utt(Some("utt_id"));
    println!("After start_utt()");

    while let true = try!(stream.is_active()) {
        while let Ok(frames) = receiver.try_recv() {
            //println!("received: {:?}", frames);
            ps_decoder.process_raw(frames, false, false);

            if start.elapsed().unwrap() > delay {
                start = SystemTime::now();

                ps_decoder.end_utt();
                match ps_decoder.get_hyp() {
                    None => println!("Not recognized"),
                    Some((hyp, _utt_id, _score)) => println!("Recognized: {}", hyp),
                }

                // Start recognizing new samples.
                ps_decoder.start_utt(Some("utt_id"));
            }
        }

    }

    try!(stream.stop());

    Ok(())
}
