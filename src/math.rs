//! Mathe relate stuff

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(compactly = false))]
#[derive(Debug, Clone, Copy)]
/// Convert from one ratio to another
pub struct RatioConverter2D<T: Into<F>, F: Into<T> + Copy> {
    ratio: (F, F),
    inverse_ratio: (F, F),
    phantom: core::marker::PhantomData<T>,
}
// use crate::prelude::{FromPatch, IntoPatch, Unwrap, UnwrapDefault};
#[allow(clippy::missing_const_for_fn)]
const impl<
    T: [const] core::ops::Add<Output = T>
        + [const] core::ops::Sub<Output = T>
        + [const] core::ops::Mul<Output = T>
        + [const] core::ops::Div<Output = T>
        + Copy
        + [const] Into<F>,
    F: [const] Into<T>
        + [const] core::ops::Add<Output = F>
        + [const] core::ops::Sub<Output = F>
        + [const] core::ops::Mul<Output = F>
        + [const] core::ops::Div<Output = F>
        + Copy,
> RatioConverter2D<T, F>
{
    /// Create a new ratio
    #[must_use]
    pub fn new(smaller: (T, T), bigger: (T, T)) -> Self {
        Self {
            ratio: (
                (smaller.0).into() / (bigger.0).into(),
                (smaller.1).into() / (bigger.1).into(),
            ),
            inverse_ratio: (
                (bigger.0).into() / (smaller.0).into(),
                (bigger.1).into() / (smaller.1).into(),
            ),
            phantom: core::marker::PhantomData,
        }
    }
    /// Convert a value from the smaller ratio to the bigger ratio
    pub fn smaller_to_bigger(&self, value: (T, T)) -> (T, T) {
        (
            ((value.0).into() * self.inverse_ratio.0).into(),
            ((value.1).into() * self.inverse_ratio.1).into(),
        )
    }
    /// Convert a value from the bigger ratio to the smaller ratio
    pub fn bigger_to_smaller(&self, value: (T, T)) -> (T, T) {
        (
            ((value.0).into() * self.ratio.0).into(),
            ((value.1).into() * self.ratio.1).into(),
        )
    }
}
