use cairo;
use gtk::DrawingArea;

// Read data each frame, push it to the ringbuffer
// implement it on a static memory???

// For now the ring buffer operates on a vec -> TODO: convert to a heap
// contigous memory
pub struct RingBuffer<T> {
    data: Vec<T>,
    read: usize,
    write: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> RingBuffer<T> {
        assert!(
            is_power_of_two(capacity),
            "Capacity of the RingBuffer must be a power of two!"
        );
        RingBuffer {
            data: Vec::with_capacity(capacity),
            read: 0,
            write: 0,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.full() {
            Err("Push failed! The RingBuffer is full!")
        } else {
            self.write += 1;
            let idx = self.mask(self.write);
            self.data[idx] = val;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.empty() {
            Err("Pop failed! The RingBuffer is empty!")
        } else {
            self.read += 1;
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
        self.write - self.read
    }

    fn mask(&self, val: usize) -> usize {
        val & (self.data.len() - 1)
    }
}

// cannot be 0 or non-power of two
pub fn is_power_of_two(val: usize) -> bool {
    (val & (val - 1)) == 0
}

pub struct Graph {
    pub data: RingBuffer<Vec<i32>>, // a ring buffer of vectors of data
    pub area: DrawingArea,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            data: RingBuffer::new(16),
            area: DrawingArea::new(),
        }
    }
}
