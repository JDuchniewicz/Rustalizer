extern crate gio;
extern crate glib;
extern crate gtk;

use rustalizer::AudioConnection;
use rustalizer::Equalizer;
use std::env;
use std::process;

mod app;

fn main() {
    // In here we have to decide on steps to undertake: parse command line arguments to display
    // either gui app or command line applet
    let args: Vec<String> = env::args().collect();
    let choice = parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // start processing backend here
    // TODO: depending on the option, change the source
    let equalizer =
        Equalizer::new(AudioConnection::File("test.wav".to_string())).unwrap_or_else(|err| {
            println!("Cannot create Equalizer backend: {}", err);
            process::exit(1);
        });
    equalizer.run();

    if choice == "GUI" {
        let application = app::GuiApp::new("MyApp");
        application.build_ui();
        application.connect_backend(&equalizer);
        application.run();
    }
}

fn parse_config(args: &[String]) -> Result<String, &str> {
    // later match it against an enum
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    Ok(args[1].clone())
}
