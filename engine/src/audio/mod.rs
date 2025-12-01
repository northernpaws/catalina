pub struct Buffer<'a, T> {
    data: &'a mut [T],
}

pub trait AudioSource<T> {
    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut Buffer<T>);
}
