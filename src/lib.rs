// this should act upon data received from the audio connection module?
// data is processed and functions are tested, then it is fed to the gui app via some kind of
// connectin?

pub enum AudioConnection {
    // TODO: decide whether use String or str
    File(String),   // plays one track and exits
    Stream(String), // Connect to a stream and play audio
}

pub struct Equalizer {
    // handle to audio file,stream etc
    connection: AudioConnection,
    status: bool,
}

impl Equalizer {
    pub fn new(connection: AudioConnection) -> Result<Equalizer, &'static str> {
        // create an audio connection and prepare it
        match connection {
            AudioConnection::File(ref _name) => Ok(Equalizer {
                connection: connection,
                status: false,
            }),
            AudioConnection::Stream(ref _name) => Ok(Equalizer {
                connection: connection,
                status: false,
            }),
        }
    }

    pub fn run(&self) -> () {
        // open the connection
        // process?
    }

    pub fn process() -> () {
        // start processing or periodically process?
        ()
    }
}
