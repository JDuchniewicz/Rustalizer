extern crate gio;
#[macro_use]
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate log;
extern crate simplelog;

mod app;
mod equalizer;
mod errors;
mod ring_buffer;
mod tui;

use anyhow::{Context, Result};
use equalizer::Equalizer;
use simplelog::*;
use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Sound visualization and (maybe equalization) for audio streams
///
/// BOOP
struct Cli {
    /// Specify display type, GUI for a GTK window based rendering, or TUI for a terminal based
    /// interface
    #[structopt(name = "mode", long, short)]
    app_mode: String,
    /// Pass the name of monitored device, obtained by running "rustalizer -q"
    #[structopt(name = "device", long, short)]
    device_name: Option<String>,
    /// Pass the name of audio host, obtained by running "rustalizer -q"
    #[structopt(name = "host", long, short)]
    host_name: Option<String>,
    /// Display available devices and hosts
    #[structopt(short, long)]
    query: bool,
    /// Number of frequency bins displayed, if empty then a common frequency binning is applied
    #[structopt(short, long)]
    bins: Option<usize>,
}

fn main() -> Result<()> {
    // In here we have to decide on steps to undertake: parse command line arguments to display
    // either gui app or command line applet
    let args = Cli::from_args();

    SimpleLogger::init(LevelFilter::Error, Config::default())
        .with_context(|| format!("Cannot set up SimpleLogger"))?;

    if args.query {
        Equalizer::query();
        process::exit(1);
    }

    // start processing backend here
    // TODO: depending on the option, change the source
    let host_name_copy = args.host_name.clone(); // TODO: must I do this dance?
    let device_name_copy = args.device_name.clone();
    let bins_copy = args.bins.clone();
    let equalizer = Rc::new(RefCell::new(
        Equalizer::new(&device_name_copy, &host_name_copy, bins_copy).with_context(|| {
            format!(
                "Cannot create Equalizer backend for host: {} and device: {}",
                device_name_copy.unwrap_or("Default".to_string()),
                host_name_copy.unwrap_or("Unknown".to_string())
            )
        })?,
    ));

    match args.app_mode.as_str() {
        "GUI" => {
            let application = app::GuiApp::new("MyApp");
            application.build_ui(equalizer.clone(), args.bins); // move the cloned rc to app closure -> now it also owns it
            equalizer
                .borrow_mut()
                .connect()
                .with_context(|| format!("cannot connect to the audio stream!"))?;
            equalizer
                .borrow()
                .play()
                .with_context(|| format!("cannot play the audio stream!"))?;
            application.run();
            Ok(())
        }
        "TUI" | _ => {
            equalizer
                .borrow_mut()
                .connect()
                .with_context(|| format!("cannot connect to the audio stream"))?;
            equalizer
                .borrow()
                .play()
                .with_context(|| format!("cannot play the audio stream!"))?;
            let mut application = tui::TerminalApp::new(equalizer)?;
            application.run(args.bins)?;
            // handle TUI stuff
            Ok(())
        }
    }
}
