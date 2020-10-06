#[derive(Copy, Clone)]
pub enum WindowType {
    Hann,
}

// per-sample window function
pub fn choose_window<T>(window: WindowType) -> impl Fn(T, usize, usize) -> T
where
    T: std::ops::Mul<f32, Output = T>,
{
    match window {
        WindowType::Hann => move |sample, idx, size| {
            let mult: f32 =
                0.5 * (1. - ((2. * std::f32::consts::PI * idx as f32 / size as f32) as f32).cos());
            sample * mult
        },
    }
}
