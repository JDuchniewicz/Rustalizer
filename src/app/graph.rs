use crate::errors::Error;
use crate::ring_buffer::RingBuffer;

use gdk::WindowExt;
use gtk::{BoxExt, ContainerExt, DrawingArea, WidgetExt};

// Read data each frame, push it to the ringbuffer
// implement it on a static memory???

pub struct Graph {
    pub data: RingBuffer<Vec<usize>>, // a ring buffer of vectors of data
    pub area: DrawingArea,
    horizontal_layout: gtk::Box,
    bins: Option<usize>,
}

impl Graph {
    pub fn new(width: i32, height: i32, bins: Option<usize>) -> Graph {
        let g = Graph {
            data: RingBuffer::new(16),
            area: DrawingArea::new(),
            horizontal_layout: gtk::Box::new(gtk::Orientation::Horizontal, 0),
            bins,
        };
        g.area.set_size_request(width, height);
        g.horizontal_layout.pack_start(&g.area, true, true, 0);
        g.horizontal_layout.set_margin_start(5);
        g
    }

    pub fn attach_to(&self, to: &gtk::Box) {
        to.add(&self.horizontal_layout);
    }

    pub fn push(&mut self, data: Vec<usize>) -> Result<(), Error> {
        info!("Received data");
        self.data.push(data)?;
        self.invalidate();
        Ok(())
    }

    // TODO: dirty algorithm for that
    pub fn draw(&mut self, ctx: &cairo::Context, width: f64, height: f64) {
        // paint background with grey
        ctx.set_source_rgb(0.5, 0.5, 0.5);
        ctx.rectangle(0., 0., width, height);
        ctx.fill();
        ctx.set_line_width(0.5);

        // Draw it 20 on 30 cells //TODO: adjust to bins number
        // go column by column altering colours and drawing up with a magnitude
        let x_incr = width / self.bins.unwrap_or(31) as f64; // TODO: default is 21?
        let y_incr = height / 30.;
        //dbg!(x_incr, y_incr);
        let y_sep = 1.;
        let x_sep = 1.;
        let mut x_pos = 0.;
        let mut y_pos;
        info!("before drawing");

        if let Ok(data) = self.data.pop() {
            for i in data {
                let mut y_ctr; // TODO: adjust scaling
                if i > 0 && i < 100 {
                    y_ctr = 1;
                } else {
                    y_ctr = i / 100;
                }
                y_pos = height - y_incr - y_sep;
                // print each column
                if i > 3000 {
                    //TODO: adjust scaling
                    y_ctr = 30;
                }
                for _ in 0..y_ctr as usize {
                    //error!("X: {} Y:{}", x_pos, y_pos);
                    // draw column
                    ctx.set_source_rgb(0., 0., 1.0);
                    ctx.rectangle(x_pos, y_pos, x_incr - x_sep, y_incr - y_sep);
                    ctx.fill();

                    y_pos -= y_sep; // what?
                                    // draw separator
                    y_pos -= y_incr;
                }
                x_pos += x_incr;
            }
        }
        info!("after drawing");
        //self.draw_labels()
        //TODO : draw labels undernath
    }

    pub fn invalidate(&self) {
        if let Some(win) = self.area.get_window() {
            info!("Invalidate called");
            let (x, y) = self
                .area
                .translate_coordinates(&self.area, 0, 0)
                .expect("Translate coordinates failed");
            let rect = gdk::Rectangle {
                x,
                y,
                width: self.area.get_allocated_width(),
                height: self.area.get_allocated_height(),
            };
            win.invalidate_rect(Some(&rect), true); // TODO: apparently invalidate does not work?
        }
    }

    /*
    fn draw_labels(&self) {

        // dara
    }
    */
}
