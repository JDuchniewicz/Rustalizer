use gio::prelude::*;
use gtk::prelude::*;

use cairo;
use gtk::{Application, ApplicationWindow, Box, Frame, Label};
use rustalizer::Equalizer;

// choose the proper application, whether console ncurses or fullfledged gui app?
pub struct GuiApp {
    application: gtk::Application,
    name: String, // name of the only window for now
}

impl GuiApp {
    // the idea is as follows: to be able to chain the calls to the app in a builder like fashion
    // and then run the run function, for now do the simpler, just move functions outside
    pub fn new(name: &'static str) -> GuiApp {
        let application =
            gtk::Application::new(Some("com.jduchniewicz.rustalizer.app"), Default::default())
                .expect("failed to initialize NewApp");
        GuiApp {
            application,
            name: name.to_owned(),
        }
    }

    pub fn build_ui(&self) -> () {
        self.application.connect_activate(|app| {
            let window = gtk::ApplicationWindow::new(app);

            let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
            vertical_layout.set_spacing(5);
            vertical_layout.set_margin_top(10);
            vertical_layout.set_margin_bottom(10);
            vertical_layout.set_margin_start(5);
            vertical_layout.set_margin_end(5);

            let area = gtk::DrawingArea::new();

            window.set_title("Rustalizer"); // lifetime issues with closures, TODO: fix this
            window.set_default_size(800, 600);

            // very very simple drawing of rectangle
            area.connect_draw(move |_w, c| {
                println!("draw");
                c.rectangle(1.0, 1.0, 100.0, 200.0);
                c.fill();
                gtk::Inhibit(false)
            });
            // instead connect a graph object which will update periodically

            let label = gtk::Label::with_mnemonic(Some("BOO")); // TODO: remove
            vertical_layout.pack_start(&label, true, true, 0);
            vertical_layout.pack_start(&area, true, true, 0);
            window.add(&vertical_layout);

            window.show_all();
        });
    }

    pub fn run(&self) -> () {
        glib::set_application_name("rustalizer");
        self.application.run(&[]);
    }

    pub fn connect_backend(&self, source: &Equalizer) -> () {
        // add the data to the graph? // TODO: should it be here? The object heirarchy needs to be
        // somehow specified and maintained
    }
}
