mod fft;
mod window;

use std::cell::Cell;
use std::sync::mpsc;
use std::thread;

enum Message {
    Raw(Vec<Cell<f32>>),
    Processed(Vec<Cell<f32>>),
    Terminate,
}

pub struct DSP {
    worker: Option<thread::JoinHandle<()>>,
    data_in_sender: mpsc::Sender<Message>, // TODO: change it to a generics, need traits?
    data_out_receiver: mpsc::Receiver<Message>,
    window: Box<dyn window::Window<i32> + Send>, // to allow for different windows at runtime
}

impl DSP {
    pub fn new() -> DSP {
        let (data_in_sender, data_in_receiver) = mpsc::channel();
        let (data_out_sender, data_out_receiver) = mpsc::channel();

        let thread = thread::spawn(move || loop {
            // This could be made async?
            let data = data_in_receiver.recv().unwrap();

            match data {
                Message::Raw(payload) => {
                    // pass to fft
                    let fft_data = fft::fft(payload);
                    // TODO: connect these two dots -> data from stream with the message passing
                    // (can I pass a mutable reference inside a message? wrap into a box?

                    // pass result to window
                    data_out_sender.send(Message::Processed(fft_data)); // TODO: change the message payload?
                }
                Message::Terminate | Message::Processed(_) => {
                    break;
                }
            }
        }); //TODO: fill the thread's processing pipeline -> receive from queue, pass through processes, push to receiver

        DSP {
            worker: Some(thread),
            data_in_sender: data_in_sender,
            data_out_receiver: data_out_receiver,
            window: Box::new(window::Hann::<i32>::new()),
        }
    }

    pub fn process(&mut self) {}

    //send method -> on callback from the application
    pub fn send(&self, data: &mut [f32]) {
        // copy the data and already extend it
        self.data_in_sender
            .send(Message::Raw(fft::extend(data, data.len())));
    }

    pub fn receive(&self) -> Option<Vec<Cell<f32>>> {
        match self.data_out_receiver.recv().unwrap() {
            Message::Processed(payload) => Some(payload),
            Message::Terminate | Message::Raw(_) => None, // will not happen?
        }
    }

    // receive method -> called from the graph update function

    // this needs to be running in a separate thread and receive from the audio core, pushing to
    // the graph
    // needs a function which will receive proper audio samples to process and store ready data in
    // a ringbuffer?
}

impl Drop for DSP {
    fn drop(&mut self) {
        debug!("Closing the DSP backend, joining thread.");

        self.data_in_sender.send(Message::Terminate).unwrap();

        if let Some(thread) = self.worker.take() {
            thread.join().unwrap();
        }
    }
}
