extern crate gio;
#[macro_use]
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate log;
extern crate simplelog;

mod app;
mod equalizer;

use equalizer::Equalizer;
use simplelog::*;
use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    app_mode: String,
    device_nr: Option<u32>,
    #[structopt(short, long)]
    query: bool,
}

fn main() {
    // In here we have to decide on steps to undertake: parse command line arguments to display
    // either gui app or command line applet
    let args = Cli::from_args();

    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    if args.query {
        Equalizer::query();
        process::exit(1);
    }

    // start processing backend here
    // TODO: depending on the option, change the source
    let equalizer = Rc::new(RefCell::new(Equalizer::new(args.device_nr).unwrap_or_else(
        |err| {
            debug!("Cannot create Equalizer backend: {}", err);
            process::exit(1);
        },
    )));

    match args.app_mode.as_str() {
        "GUI" => {
            let application = app::GuiApp::new("MyApp");
            application.build_ui(equalizer.clone()); // move the cloned rc to app closure -> now it also owns it
            equalizer.borrow_mut().connect();
            equalizer.borrow().play();
            application.run();
            //application.connect_backend(&equalizer);
        }
        _ => {
            debug!("Unknown option, defaulting to console!"); //TODO: fix later once console is supported
            process::exit(1);
        }
    }
}
