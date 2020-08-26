pub struct DSP {}

impl DSP {
    pub fn new() -> DSP {
        DSP {}
    }

    // this needs to be running in a separate thread and receive from the audio core, pushing to
    // the graph
    // needs a function which will receive proper audio samples to process and store ready data in
    // a ringbuffer?
}
