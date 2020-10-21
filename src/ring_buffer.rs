use crate::errors::{BufferOp, Error};
use std::num::Wrapping;

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

    pub fn push(&mut self, val: T) -> Result<(), Error> {
        if self.full() {
            Err(Error::BufferOperation(BufferOp::Push))
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

    pub fn pop(&mut self) -> Result<T, Error> {
        if self.empty() {
            Err(Error::BufferOperation(BufferOp::Pop))
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

    pub fn top(&self) -> Result<T, Error> {
        if self.empty() {
            Err(Error::BufferOperation(BufferOp::Pop))
        } else {
            let idx = self.mask(self.read.wrapping_add(1));
            debug!(
                "RB top, idx {}, write {} read {}",
                idx,
                self.write,
                self.read.wrapping_add(1)
            );
            Ok(self.data[idx].clone())
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
