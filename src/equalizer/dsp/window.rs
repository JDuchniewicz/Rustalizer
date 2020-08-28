pub trait Window<T> {
    fn window(&self);
}

pub struct Hann<T> {
    // parameters
    dummy: Vec<T>,
}

impl<T> Window<T> for Hann<T> {
    fn window(&self) {
        //pass through
    }
}

impl<T> Hann<T> {
    pub fn new() -> Hann<T> {
        Hann { dummy: Vec::new() }
    }
}
