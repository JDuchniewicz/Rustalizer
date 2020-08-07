// this should act upon data received from the audio connection module?
// data is processed and functions are tested, then it is fed to the gui app via some kind of
// connectin?

enum AudioConnection {
    File(String),   // plays one track and exits
    Stream(String), // Connect to a stream and play audio
}

pub struct Equalizer {
    // handle to audio file,stream etc
    connection: AudioConnection,
    status: bool,
}

impl Equalizer {
    pub fn new() -> Result<Equalizer, &'static str> {
        // create an audio connection and prepare it
        Ok(Equalizer {
            connection: AudioConnection::File("dupa".to_string()),
            status: false,
        })
    }

    pub fn process() -> () {
        // start processing or periodically process?
        ()
    }
}

pub fn hey() {
    println!("YELLO");
}
