use std::cell::Cell;

// receives already extended vector of both real and imaginary values
// requires the input data to be a power of two, lest wrong indexing happens!
pub fn fft<T>(mut data: Vec<Cell<T>>) -> Vec<Cell<T>>
where
    T: Copy
        + std::cmp::PartialEq
        + std::convert::From<f32>
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>,
    f32: std::convert::From<T>,
{
    // in this function compute the FFT
    // first change encoding for Danielson-Lanczos
    // then do the algorithm and return by reference
    let n: usize = data.len();
    info!("FFT len: {}", n);
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
        theta = 2.0 * std::f32::consts::PI / mmax as f32; // here sign decides whether 1 or -1 (IFFT)
        wtemp = (theta * 0.5).sin();
        wpr = -2.0 * wtemp * wtemp;
        wpi = theta.sin();
        wr = 1.0;
        wi = 0.0;

        for m in (1..mmax).step_by(2) {
            for i in (m..=n).step_by(istep) {
                j = i + mmax;
                //                debug!(
                //                    "Values: i {} j {} m {} mmax {} istep {}",
                //                    i, j, m, mmax, istep
                //                );
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

// finds the nearest power of 2 the length satisfies and zero-extends the buffer
// after preparing the data for FFT (interleaving)
pub fn prepare_data<T>(
    data: &[T],
    len: usize,
    window_func: impl Fn(T, usize, usize) -> T,
) -> Vec<Cell<T>>
where
    T: Copy + Default,
{
    let as_float: f32 = len as f32;
    let nearest_two_pow = as_float.log2().floor() + 1.0;
    let new_len = 2 << nearest_two_pow as usize;
    info!("Prepare data for FFT, len {}", new_len);
    let mut extended = Vec::with_capacity(new_len);
    for i in 0..len {
        extended.push(Cell::new(window_func(data[i], i, len)));
        extended.push(Cell::new(T::default()));
    }
    for _ in 2 * len..new_len {
        extended.push(Cell::new(T::default()));
    }
    extended
}

// TODO: write docs
// converts the input FFT data to num_bins size
pub fn to_bins<T>(data: Vec<Cell<T>>, num_bins: usize) -> Vec<usize>
// TODO: probably need a result for checking, the buffer should be discarded asap?
// INCOMING DATA IS 0?>????
where
    T: Copy
        + std::fmt::Display
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::convert::From<f32>,
    f32: std::convert::From<T>,
{
    // care only to N/2 frequencies
    // real intreleaved with imaginary
    // 0 is the dc offset
    // TODO: probably need a place for magic numbers definitions? FFT constants etc
    //debug!("to bins data in len {}", data.len());
    if data.len() > 44100 {
        error!(
            "The input data was greater than the sampling rate, probably CPAL hiccup - ignoring"
        );
        return Vec::new();
    }
    let bin_width: usize = ((data.len() / 4) as f32 / num_bins as f32).ceil() as usize;
    let mut bin_idx: usize = 1; // index from 1 but store at 0
    let mut freq_magnitude: f32;
    let mut bins = Vec::<f32>::with_capacity(num_bins);
    for _ in 0..num_bins {
        bins.push(0.);
    }

    info!("bin_width {} data.len() {}", bin_width, data.len());
    // Here they need to be thrown into appropriate bin
    for i in (0..data.len() / 2).step_by(2) {
        if i / 2 > bin_idx * bin_width {
            bin_idx += 1;
        }
        //debug!("incoming data for i {} is {}", i, data[i].get());
        freq_magnitude = Into::<f32>::into(
            data[i].get() * data[i].get() + data[i + 1].get() * data[i + 1].get(),
        );
        //debug!("the magnitude at idx {} is {}", i, freq_magnitude);
        bins[bin_idx - 1] += freq_magnitude;
    }

    // finally they should be normalized
    let normalized = bins
        .iter()
        .map(|&b| b as usize / bin_width)
        .collect::<Vec<usize>>();
    for i in 0..normalized.len() {
        info!("NormBin {} value {}", i, normalized[i]);
    }
    normalized
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
