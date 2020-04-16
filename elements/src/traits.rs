//! Helper traits, used internally.

/// Similar to `Extend` in standard library for builder like types.
///
/// used mostly with views that containes multiple items such as `Flexbox`
pub trait ExtendBuilder<A> {
    fn extend<T>(self, iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}
