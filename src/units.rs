pub struct Seconds<T>(pub T);
impl From<instant::Duration> for Seconds<f32> {
    fn from(dt: instant::Duration) -> Self {
        Seconds(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9)
    }
}
