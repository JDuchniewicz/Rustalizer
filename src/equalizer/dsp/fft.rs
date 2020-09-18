use std::cell::Cell;

// receives already extended vector of both real and imaginary values
pub fn fft<T>(mut data: Vec<Cell<T>>) -> Vec<Cell<T>>
where
    T: Copy
        + Default
        + std::convert::From<f32>
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>,
    f32: std::convert::From<T>,
{
    // in this function compute the FFT
    // first change encoding for Danielson-Lanczos
    // then do the algorithm and return by reference
    let n: usize = data.len();
    debug!("FFT len: {}", n);
    let mut j = 0;
    let mut m: usize;
    for i in (0..n / 2).step_by(2) {
        if j > i {
            data.swap(j, i); // swap real
            data.swap(j + 1, i + 1); // swap complex
        }

        if (j / 2) < (n / 4) {
            data.swap(n - (i + 2), n - (j + 2));
            data.swap(n - (i + 2) + 1, n - (j + 2) + 1);
        }

        m = n / 2;
        while m >= 2 && j >= m {
            j -= m;
            m = m / 2;
        }
        j += m;
    }

    // Danielson-Lanczos
    let mut mmax: usize = 2;
    let (mut istep, mut theta, mut wtemp, mut wpr, mut wpi, mut wr, mut wi, mut tempr, mut tempi): (
        usize,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    );

    while n > mmax {
        istep = mmax << 1;
        theta = 2.0 * std::f32::consts::PI / mmax as f32; // here signum decides whether 1 or -1 (IFFT_
        wtemp = (theta * 0.5).sin();
        wpr = -2.0 * wtemp * wtemp;
        wpi = theta.sin();
        wr = 1.0;
        wi = 0.0;

        for m in (1..mmax).step_by(2) {
            for i in (m..=n).step_by(istep) {
                j = i + mmax; // TODO: this goes out of range?????!!! apparently for non-powers of 2 it will fail
                debug!(
                    "Values: i {} j {} m {} mmax {} istep {}",
                    i, j, m, mmax, istep
                );
                tempr = wr * Into::<f32>::into(data[j - 1].get())
                    - wi * Into::<f32>::into(data[j].get());
                tempi = wr * Into::<f32>::into(data[j].get())
                    + wi * Into::<f32>::into(data[j - 1].get());
                data[j - 1].set(data[i - 1].get() - Into::<T>::into(tempr));
                data[j].set(data[i].get() - Into::<T>::into(tempi));
                data[i - 1].set(data[i - 1].get() + Into::<T>::into(tempr));
                data[i].set(data[i].get() + Into::<T>::into(tempi));
            }
            wtemp = wr;
            wr = wtemp * wpr - wi * wpi + wr;
            wi = wi * wpr + wtemp * wpi + wi;
        }
        mmax = istep;
    }

    data
}

// extends the array with samples of value 0
pub fn extend<T>(data: &mut [T], len: usize) -> Vec<Cell<T>>
where
    T: Copy + Default,
{
    let mut extended = Vec::with_capacity(2 * len);
    for i in 0..len {
        extended.push(Cell::new(data[i]));
        extended.push(Cell::new(T::default()));
    }
    extended
}

// tests on floats TODO: add tests for i16?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_size() {
        const capacity: usize = 1024;
        let mut vec = Vec::with_capacity(capacity);
        for i in 0..capacity / 2 {
            vec.push(Cell::new(i as f32));
            vec.push(Cell::new(0.0));
        }
        let vec_len = vec.len();
        let transformed = fft(vec);
        assert_eq!(vec_len, transformed.len());
    }

    #[test]
    fn non_zero() {
        const capacity: usize = 1024;
        let mut vec = Vec::with_capacity(capacity);
        for i in 0..capacity / 2 {
            vec.push(Cell::new(i as f32));
            vec.push(Cell::new(0.0));
        }
        let transformed = fft(vec);

        let mut onlyZeroes: bool = true;
        for i in transformed.into_iter() {
            println!("{:?}", i.get());
            if i.get() != 0.0 {
                onlyZeroes = false;
            }
        }

        assert_eq!(onlyZeroes, false);
    }
}
