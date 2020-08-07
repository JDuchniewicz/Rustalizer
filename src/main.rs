extern crate gio;
extern crate glib;
extern crate gtk;

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

    if choice == "GUI" {
        let application = app::GuiApp::new("MyApp");
        application.build_ui();
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
