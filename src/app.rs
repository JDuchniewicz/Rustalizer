use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

// choose the proper application, whether console ncurses or fullfledged gui app?
pub struct GuiApp {
    application: gtk::Application,
    name: String, // name of the only window for now
}

impl GuiApp {
    // the idea is as follows: to be able to chain the calls to the app in a builder like fashion
    // and then run the run function, for now do the simpler, just move functions outside
    pub fn new(name: &str) -> GuiApp {
        let application =
            Application::new(Some("com.jduchniewicz.rustalizer.app"), Default::default())
                .expect("failed to initialize NewApp");
        GuiApp {
            application,
            name: name.to_owned(),
        }
    }

    pub fn build(&self) -> () {
        self.application.connect_activate(|app| {
            let window = ApplicationWindow::new(app);
            window.set_title("RANDOM"); // how to pass the name from constructor????
            window.set_default_size(800, 600);

            let button = Button::with_label("Click me!");
            button.connect_clicked(|_| {
                println!("clicked!");
            });
            window.add(&button);

            window.show_all();
        });
    }

    pub fn run(&self) -> () {
        self.application.run(&[]);
    }
}
