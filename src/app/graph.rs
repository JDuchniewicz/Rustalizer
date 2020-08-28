use cairo::Context;
use gtk::{BoxExt, ContainerExt, DrawingArea, WidgetExt};
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
            Ok(self.data.swap_remove(idx))
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
    pub data: RingBuffer<Vec<i32>>, // a ring buffer of vectors of data
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

    pub fn push(&mut self, data: Vec<i32>) {
        // push data for drawing into the buffer
    }

    pub fn draw(&self, ctx: &cairo::Context, width: f64, height: f64) {}
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
