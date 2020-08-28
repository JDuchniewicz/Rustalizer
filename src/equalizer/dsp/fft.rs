pub struct FFT<T> {
    data: Vec<T>,
}

impl<T> FFT<T> {
    pub fn new() -> FFT<T> {
        FFT { data: Vec::new() }
    }
}
