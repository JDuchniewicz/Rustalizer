use cairo::Context;
use gdk::WindowExt;
use gtk::{BoxExt, ContainerExt, DrawingArea, WidgetExt};
use std::mem;
use std::num::Wrapping;

// Read data each frame, push it to the ringbuffer
// implement it on a static memory???

// For now the ring buffer operates on a vec -> TODO: convert to a heap
// contigous memory
pub struct RingBuffer<T> {
    data: Vec<T>,
    read: usize,
    write: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> RingBuffer<T> {
        assert!(
            is_power_of_two(capacity),
            "Capacity of the RingBuffer must be a power of two!"
        );
        RingBuffer {
            data: vec![T::default(); capacity],
            read: 0,
            write: 0,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.full() {
            Err("Push failed! The RingBuffer is full!")
        } else {
            //self.write += 1;
            self.write = self.write.wrapping_add(1);
            let idx = self.mask(self.write);
            self.data[idx] = val;
            debug!(
                "RB push, idx {}, write {} read {}",
                idx, self.write, self.read
            );
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.empty() {
            Err("Pop failed! The RingBuffer is empty!")
        } else {
            //self.read += 1;
            self.read = self.read.wrapping_add(1);
            let idx = self.mask(self.read);
            debug!(
                "RB pop, idx {}, write {} read {}",
                idx, self.write, self.read
            );
            Ok(std::mem::replace(&mut self.data[idx], T::default()))
        }
    }

    pub fn full(&self) -> bool {
        self.size() == self.data.capacity()
    }

    pub fn empty(&self) -> bool {
        self.read == self.write
    }

    pub fn size(&self) -> usize {
        (Wrapping(self.write) - Wrapping(self.read)).0
    }

    fn mask(&self, val: usize) -> usize {
        val & (self.data.capacity() - 1)
    }
}

// cannot be 0 or non-power of two
pub fn is_power_of_two(val: usize) -> bool {
    (val & (val - 1)) == 0
}

pub struct Graph {
    pub data: RingBuffer<Vec<usize>>, // a ring buffer of vectors of data
    pub area: DrawingArea,
    horizontal_layout: gtk::Box,
}

impl Graph {
    pub fn new() -> Graph {
        let g = Graph {
            data: RingBuffer::new(16),
            area: DrawingArea::new(),
            horizontal_layout: gtk::Box::new(gtk::Orientation::Horizontal, 0),
        };
        g.horizontal_layout.pack_start(&g.area, true, true, 0);
        g.horizontal_layout.set_margin_start(5);
        g
    }

    pub fn attach_to(&self, to: &gtk::Box) {
        to.add(&self.horizontal_layout);
    }

    pub fn push(&mut self, data: Vec<usize>) -> Result<(), &'static str> {
        error!("Received data");
        if let Ok(_) = self.data.push(data) {
            self.invalidate();
            Ok(())
        } else {
            Err("Failed to push data to Graph")
        }
    }

    pub fn draw(&mut self, ctx: &cairo::Context, width: f64, height: f64) {
        // paint background with grey
        ctx.set_source_rgb(0.5, 0.5, 0.5);
        ctx.rectangle(0., 0., width, height);
        ctx.fill();
        ctx.set_line_width(0.5);

        // Draw it 20 on 30 cells
        // go column by column altering colours and drawing up with a magnitude
        let x_incr = width / 20.;
        let y_incr = height / 30.;
        let y_sep = 1.;
        let x_sep = 1.;
        let mut x_pos = 0.;
        let mut y_pos = 0.;
        error!("before drawing");

        if let Ok(data) = self.data.pop() {
            error!("Drawing");
            for i in data.into_iter() {
                let mut y_ctr = 0.;
                // print each column
                // TODO: dirty algorithm for that
                if i > 30 {
                    y_ctr = 30.;
                }
                for _ in 0..y_ctr as usize {
                    // draw column
                    ctx.set_source_rgb(0., 0., 1.0);
                    ctx.rectangle(x_pos, y_pos, x_incr - x_sep, y_incr - y_sep);
                    ctx.fill();

                    y_pos += y_sep; // what?
                                    // draw separator
                    y_pos += y_ctr;
                }
                x_pos += x_incr;
            }
        }
        error!("after drawing");
    }

    pub fn invalidate(&self) {
        if let Some(win) = self.area.get_window() {
            error!("Invalidate called");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn non_two_power() {
        RingBuffer::<i32>::new(3);
    }

    #[test]
    fn pop_empty() {
        let mut buf = RingBuffer::<i32>::new(2);
        assert!(
            buf.pop().is_err(),
            "The RingBuffer should not allow for popping when it is empty!"
        );
    }

    #[test]
    fn push_full() {
        let mut buf = RingBuffer::<i32>::new(2);
        buf.push(1);
        buf.push(2);
        debug!("{}", buf.size());
        assert!(
            buf.push(3).is_err(),
            "The RingBuffer should not allow for pushing when it is full!"
        );
    }
}
