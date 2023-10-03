#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, Sink};

    #[test]
    fn play_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("assets/TETRIS.mp3").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();        
        sink.append(source);

        sink.set_speed(2.0);

        sink.sleep_until_end();
    }
}