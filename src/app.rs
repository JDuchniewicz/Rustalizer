use gio::prelude::*;
use gtk::prelude::*;

mod graph;

use crate::equalizer::Equalizer;
use gtk::{Application, ApplicationWindow, Box, Frame, Label};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

// TODO: add config file with configs?
const UPDATE_TIMEOUT: u64 = 500; // ms

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

    fn setup_timeout(equalizer: &Rc<RefCell<Equalizer>>, graph: &Rc<RefCell<graph::Graph>>) {
        // TODO: big refactor once it works, make it all generic properly!
        // new thread for updating feeding graph with data obtained from equalizer
        let (ready_tx, ready_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(clone!(@strong ready_tx => move || loop {
            let sleep_duration = Duration::from_millis(UPDATE_TIMEOUT);
            thread::sleep(sleep_duration);
            ready_tx
                .send(false)
                .expect("Failed to send data through graph refresh channel");
        }));

        ready_rx.attach(
            None,
            clone!(@strong graph, @weak equalizer => @default-panic, move |_: bool| { // TODO: I am not sure I understand why 'weak' graph failed and 'strong' is ok
                info!("Receiving data from equalizer for graph");
                // Test FFT workings and why it hangs here after uncommenting equalizer code
                // Rudimentary graph drawing and updating
                // Understand WTF is going on with these references and cloning
                //
                if let Some(payload) = equalizer.borrow().get_processed_samples() {
                    // This is raw FFT, could be formatted better -> 20 frequency bins and unwrap
                    // from cell
                    graph.borrow_mut().push(payload);
                }
                glib::Continue(true)
            }),
        );
    }

    // This builds the general UI of the application (for now also the main UI - equalizer graph)
    pub fn build_ui(&self, equalizer: Rc<RefCell<Equalizer>>) -> () {
        self.application.connect_activate(move |app| {
            let window = gtk::ApplicationWindow::new(app);

            const XSIZE: i32 = 800;
            const YSIZE: i32 = 600;
            const XMARGIN: i32 = 5;
            const YMARGIN: i32 = 10;

            let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
            vertical_layout.set_spacing(5);
            vertical_layout.set_margin_top(YMARGIN);
            vertical_layout.set_margin_bottom(YMARGIN);
            vertical_layout.set_margin_start(XMARGIN);
            vertical_layout.set_margin_end(XMARGIN);

            //            let area = gtk::DrawingArea::new(); // no need for a global drawing area, each graph
            //            implements their own
            window.set_title("Rustalizer"); // lifetime issues with closures, TODO: fix this
            window.set_default_size(XSIZE, YSIZE);

            let equalizer_graph = graph::Graph::new(XSIZE - 2 * XMARGIN, YSIZE - 2 * YMARGIN);
            // connect refreshing context to gtk
            equalizer_graph.attach_to(&vertical_layout);
            // share out the graph object, now it is Rc
            let equalizer_graph = GuiApp::connect_graph(equalizer_graph);

            GuiApp::setup_timeout(&equalizer, &equalizer_graph);

            /*
            // very very simple drawing of rectangle
            area.connect_draw(move |_w, c| {
                c.rectangle(1.0, 1.0, 100.0, 200.0);
                c.fill();
                gtk::Inhibit(false)
            });


            let label = gtk::Label::with_mnemonic(Some("BOO")); // TODO: remove
            vertical_layout.pack_start(&label, true, true, 0);
            */
            //vertical_layout.pack_start(&area, true, true, 0);
            window.add(&vertical_layout);

            window.show_all();
        });
    }

    pub fn run(&self) -> () {
        glib::set_application_name("rustalizer");
        self.application.run(&[]);
    }

    fn connect_graph(graph: graph::Graph) -> Rc<RefCell<graph::Graph>> {
        let area = graph.area.clone();
        let graph = Rc::new(RefCell::new(graph));
        area.connect_draw(
            clone!(@weak graph => @default-return gtk::Inhibit(false), move |w, c| {
                graph.borrow_mut().draw(
                    c,
                    f64::from(w.get_allocated_width()),
                    f64::from(w.get_allocated_height()),
                );
                gtk::Inhibit(false)
            }),
        );
        graph
    }
}
