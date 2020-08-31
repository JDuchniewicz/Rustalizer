pub struct FFT<T> {
    // TODO: this probably does not need to be a struct :)
    data: Vec<T>,
}

impl<T> FFT<T> {
    pub fn new() -> FFT<T> {
        FFT { data: Vec::new() }
    }

    // the input array has to be prepared - filled with complex samples (interleaved)

    pub fn fft(data: &mut [T], len: i32)
    where
        T: Copy,
    {
        // in this function compute the FFT
        // first change encoding for Danielson-Lanczos
        // then do the algorithm and return by reference
        let n = 2 * len;
        let mut j = 1;
        let mut m: i32;
        for i in (1..n).step_by(2) {
            if j > i {
                // swapping
            }

            if (j / 2) < (n / 4) {
                // swapping
            }

            m = n / 2;
            while m >= 2 && j >= m {
                j -= m;
                m = m / 2;
            }
            j += m;
        }

        // now lanczos
    }

    fn swap(data: &mut [T], what: usize, with: usize)
    where
        T: Copy,
    {
        let tmp = data[what];
        data[what] = data[with];
        data[with] = tmp;
    }
}
