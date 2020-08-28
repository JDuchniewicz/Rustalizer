mod fft;
mod window;

use std::sync::mpsc;
use std::thread;

enum Message {
    Data(Vec<i32>),
    Terminate,
}

pub struct DSP {
    worker: Option<thread::JoinHandle<()>>,
    data_in_sender: mpsc::Sender<Message>, // TODO: change it to a generics, need traits?
    data_out_receiver: mpsc::Receiver<Message>,
    fft: fft::FFT<i32>,
    window: Box<dyn window::Window<i32>>, // to allow for different windows at runtime
}

impl DSP {
    pub fn new() -> DSP {
        let (data_in_sender, data_in_receiver) = mpsc::channel();
        let (data_out_sender, data_out_receiver) = mpsc::channel();

        let thread = thread::spawn(move || loop {
            // This could be made async?
            let data = data_in_receiver.recv().unwrap();

            match data {
                Message::Data(_) => {
                    // pass to fft
                    // pass result to window
                    data_out_sender.send(data);
                }
                Message::Terminate => {
                    break;
                }
            }
        }); //TODO: fill the thread's processing pipeline -> receive from queue, pass through processes, push to receiver

        DSP {
            worker: Some(thread),
            data_in_sender: data_in_sender,
            data_out_receiver: data_out_receiver,
            fft: fft::FFT::<i32>::new(),
            window: Box::new(window::Hann::<i32>::new()),
        }
    }

    pub fn process(&mut self) {}

    //send method -> on callback from the application
    pub fn send(&self, data: Vec<i32>) {
        self.data_in_sender.send(Message::Data(data));
    }

    pub fn receive(&self) -> Option<Vec<i32>> {
        // TODO: consolidate it with the update function of graph
        match self.data_out_receiver.recv().unwrap() {
            Message::Data(payload) => Some(payload),
            Message::Terminate => None, // will not happen?
        }
    }

    // receive method -> called from the graph update function

    // this needs to be running in a separate thread and receive from the audio core, pushing to
    // the graph
    // needs a function which will receive proper audio samples to process and store ready data in
    // a ringbuffer?
    //
    // MAYBE NEED TWO DISTINCT MPSC QUEUES? ONE FOR PUSHING ONE FOR RECEIVING :)
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
