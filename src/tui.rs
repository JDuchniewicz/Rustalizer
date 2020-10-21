use crate::equalizer::Equalizer;
use crate::errors::Error;
use crate::ring_buffer::RingBuffer;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    cell::RefCell,
    io::{stdout, Write},
    rc::Rc,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders},
    Frame, Terminal,
};

const TICK_RATE: u64 = 100;

const NUMERIC_FREQS: [&str; 32] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
];

const REGULAR_FREQS: [&str; 31] = [
    "20 Hz", "25 Hz", "31 Hz", "40 Hz", "50 Hz", "63 Hz", "80 Hz", "100 Hz", "125 Hz", "160 Hz",
    "200 Hz", "250 Hz", "315 Hz", "400 Hz", "500 Hz", "630 Hz", "800 Hz", "1 kHz", "1.2 kHz",
    "1.6 kHz", "2 kHz", "2.5 kHz", "3.1 kHz", "4 kHz", "5 kHz", "6.3 kHz", "8 kHz", "10 kHz",
    "12 kHz", "16 kHz", "20 kHz",
];

enum IEvent<E> {
    Input(E),
    Tick,
}

pub struct TerminalApp {
    terminal: Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>, // TODO: add crossplatform-ness, no function specializations in Rust so have to come up with something else
    equalizer: Rc<RefCell<Equalizer>>,
    data: RingBuffer<Vec<usize>>,
    // store the equalizer Rc for receiving data
}

impl TerminalApp {
    pub fn new(equalizer: Rc<RefCell<Equalizer>>) -> Result<TerminalApp, Error> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let data = RingBuffer::new(16);
        Ok(TerminalApp {
            terminal,
            equalizer,
            data,
        })
    }

    pub fn run(&mut self, bins: Option<usize>) -> Result<(), Error> {
        let mut custom_bins: bool = false;
        if let Some(bins) = bins {
            assert!(bins < 31, "There cannot be more than 31 custom bins");
            custom_bins = true;
        }
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

        // prepare current batch to show
        let mut last_batch = Vec::new();

        loop {
            // paint last frame
            let labels: Vec<(&str, u64)> = last_batch
                .clone()
                .iter()
                .enumerate()
                .map(|(index, val)| {
                    let label_str;
                    if custom_bins {
                        label_str = NUMERIC_FREQS[index];
                    } else {
                        label_str = REGULAR_FREQS[index];
                    }
                    let label_val = *val as u64;
                    (label_str, label_val)
                })
                .collect();
            self.terminal.draw(|f| draw(f, &labels))?;

            match event_rx.recv()? {
                IEvent::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            self.terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        self.terminal.show_cursor()?;
                        break;
                    }
                    _ => {}
                },
                IEvent::Tick => {
                    // obtain new data and update last_batch
                    if let Some(new_batch) = self.equalizer.borrow().get_processed_samples() {
                        self.data.push(new_batch)?;
                    }

                    match self.data.pop() {
                        Err(error) => {
                            debug!("{:?}", error);
                        }
                        Ok(replacement) => {
                            last_batch = replacement;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn draw<B>(f: &mut Frame<B>, data: &[(&str, u64)])
where
    B: tui::backend::Backend,
{
    let graph = BarChart::default()
        .block(Block::default().title("Rustalizer").borders(Borders::ALL))
        .bar_width(3)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
        .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .data(data);
    f.render_widget(graph, f.size()); // can add multiple parallel ones
}
