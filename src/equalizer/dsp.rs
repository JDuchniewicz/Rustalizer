mod fft;
mod window;

use std::cell::Cell;
use std::sync::mpsc;
use std::thread;

enum Message {
    Raw(Vec<Cell<f32>>),
    Processed(Vec<usize>),
    Terminate,
}

pub struct DSP {
    worker: Option<thread::JoinHandle<()>>,
    data_in_sender: mpsc::Sender<Message>, // TODO: change it to a generics, need traits?
    data_out_receiver: mpsc::Receiver<Message>,
    window: Box<dyn window::Window<f32> + Send>, // to allow for different windows at runtime
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
                    info!("Received data for processing in DSP");
                    // window the data prior to FFTing
                    // pass to fft
                    let fft_data = fft::fft(payload);

                    // bin the processed samples to several bins
                    let binned = fft::to_bins(fft_data, 20); // TODO: magic numbers!
                    data_out_sender.send(Message::Processed(binned)); // TODO: change the message payload?
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
            window: Box::new(window::Hann::<f32>::new()),
        }
    }

    //send method -> on callback from the application
    pub fn send(&self, data: &[f32]) {
        // copy the data and already extend it
        info!("Sending data to DSP mpsc");
        self.data_in_sender
            .send(Message::Raw(fft::prepare_data(data, data.len())))
            .expect("Could not send data via MPSC from the CPAL core");
    }

    pub fn receive(&self) -> Option<Vec<usize>> {
        match self.data_out_receiver.recv().unwrap() {
            Message::Processed(payload) => Some(payload),
            Message::Terminate | Message::Raw(_) => None, // will not happen?
        }
    }
}

impl Drop for DSP {
    fn drop(&mut self) {
        info!("Closing the DSP backend, joining thread.");

        self.data_in_sender.send(Message::Terminate).unwrap();

        if let Some(thread) = self.worker.take() {
            thread.join().unwrap();
        }
    }
}
