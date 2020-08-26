extern crate gio;
extern crate glib;
extern crate gtk;

mod app;
mod equalizer;

use equalizer::Equalizer;
use std::process;
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
    println!("{:?}", args);

    if args.query {
        Equalizer::query();
        process::exit(1);
    }

    // start processing backend here
    // TODO: depending on the option, change the source
    let equalizer = Equalizer::new(args.device_nr).unwrap_or_else(|err| {
        println!("Cannot create Equalizer backend: {}", err);
        process::exit(1);
    });

    match args.app_mode.as_str() {
        "GUI" => {
            let application = app::GuiApp::new("MyApp");
            application.build_ui();
            application.connect_backend(&equalizer);
            application.run();
        }
        _ => {
            println!("Unknown option, defaulting to console!"); //TODO: fix later once console is supported
            process::exit(1);
        }
    }
}
