use crate::equalizer::Equalizer;
use crate::errors::Error;

use crossterm::event::{self, Event};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::Terminal;

const TICK_RATE: u64 = 250;

enum IEvent<E> {
    Input(E),
    Tick,
}

pub struct TerminalApp {
    terminal: Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>, // TODO: add crossplatform-ness, no function specializations in Rust so have to come up with something else
    equalizer: Rc<RefCell<Equalizer>>,
    // store the equalizer Rc for receiving data
}

impl TerminalApp {
    pub fn new(equalizer: Rc<RefCell<Equalizer>>) -> Result<TerminalApp, Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(TerminalApp {
            terminal,
            equalizer,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let (event_tx, event_rx) = mpsc::channel();

        // spawn the event transmitting thread
        let tick_rate = Duration::from_millis(TICK_RATE);

        // poll for events, if none just send a tick event
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if event::poll(timeout).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        event_tx.send(IEvent::Input(key)).unwrap();
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    event_tx.send(IEvent::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        /*
        loop {
            // paint last frame
            self.terminal.draw(|f| draw(f));
        }
        // match the events in the app main loop
        // Add special events handling

        // loop tha hoop
        // */
        Ok(())
    }

    //fn on_tick(
}

//pub fn draw<B>
