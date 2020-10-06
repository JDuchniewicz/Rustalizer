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

use anyhow::{Context, Result};
use equalizer::Equalizer;
use simplelog::*;
use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    app_mode: String,
    device_name: Option<String>,
    host_name: Option<String>,
    #[structopt(short, long)]
    query: bool,
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
    let equalizer = Rc::new(RefCell::new(
        Equalizer::new(&device_name_copy, &host_name_copy).with_context(|| {
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
            application.build_ui(equalizer.clone()); // move the cloned rc to app closure -> now it also owns it
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
        _ => {
            error!("Unknown option, defaulting to console!"); //TODO: fix later once console is supported
            process::exit(1);
        }
    }
}
