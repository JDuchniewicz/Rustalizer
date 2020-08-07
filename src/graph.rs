use cairo;
use gtk::{DrawingArea};

// Read data each frame, push it to the ringbuffer
// implement it on a static memory???

pub struct RingBuffer<T> {
    data: Vec<T>,
    start: usize,
}

impl<T> RingBuffer<T> {
    

}

pub struct Graph {
    pub data: RingBuffer<Vec<i32>>, // a ring buffer of vectors of data
    pub area: DrawingArea,
}

impl Graph {
    pub fn new() -> Graph {
        Graph()
    }
}
