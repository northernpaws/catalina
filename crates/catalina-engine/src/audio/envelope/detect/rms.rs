use super::{Detect, Detector};
use crate::audio::frame::Frame;
use crate::audio::rms;
use crate::core::ring_buffer;

impl<F, S> Detect<F> for rms::Rms<F, S>
where
    F: Frame,
    S: ring_buffer::Slice<Element = F::Float> + ring_buffer::SliceMut,
{
    type Output = F::Float;
    fn detect(&mut self, frame: F) -> Self::Output {
        self.next(frame)
    }
}

impl<F, S> Detector<F, rms::Rms<F, S>>
where
    F: Frame,
    S: ring_buffer::Slice<Element = F::Float> + ring_buffer::SliceMut,
{
    /// Construct a new **Rms** **Detector**.
    ///
    /// ### Required Features
    ///
    /// - When using `dasp_envelope`, this item requires the **rms** feature to be enabled.
    /// - When using `dasp`, this item requires the **envelope-rms** feature to be enabled.
    pub fn rms(buffer: ring_buffer::Fixed<S>, attack_frames: f32, release_frames: f32) -> Self {
        let rms = rms::Rms::new(buffer);
        Self::new(rms, attack_frames, release_frames)
    }
}
