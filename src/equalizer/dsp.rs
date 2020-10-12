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
    window_type: window::WindowType, // there is an idea for runtime-changeable window_type
                                     // add windowing function closure?
}

impl DSP {
    pub fn new(bins: Option<usize>) -> DSP {
        let (data_in_sender, data_in_receiver) = mpsc::channel();
        let (data_out_sender, data_out_receiver) = mpsc::channel();

        let thread = thread::spawn(move || loop {
            // This could be made async?
            let data = data_in_receiver.recv().unwrap();

            match data {
                Message::Raw(payload) => {
                    // TODO: can separate L/R channels? try some more advanced stuff later?
                    info!("Received data for processing in DSP");
                    // pass to fft
                    let fft_data = fft::fft(payload);

                    // bin the processed samples to several bins
                    let binned = fft::to_bins(fft_data, bins);
                    if binned.is_ok() {
                        if let Err(err) = data_out_sender.send(Message::Processed(binned.unwrap()))
                        {
                            error!("Failed to send data to DSP: {}", err);
                        }
                    } else {
                        error!("{}", binned.err().unwrap());
                    }
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
            window_type: window::WindowType::Hann,
        }
    }

    //send method -> on callback from the application
    pub fn send(&self, data: &[f32]) {
        // copy the data and already extend it
        info!("Sending data to DSP mpsc");
        self.data_in_sender
            .send(Message::Raw(fft::prepare_data(
                data,
                data.len(),
                window::choose_window(self.window_type.clone()), // window the data prior to FFTing (TODO: maybe some kind of composable pipeline of actions? it would make it easier in the future)
            )))
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
