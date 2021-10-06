use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use rdev::{listen, Event, EventType, Key};

fn main() {
    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn print_if_j(key: Key) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("./audio/click1.wav").unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    if key == Key::KeyJ {
        sink.append(source);
        sink.sleep_until_end();
    }
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => print_if_j(key),
        _ => (),
    }
}