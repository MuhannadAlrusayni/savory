//! Helper traits commonly used to make API consistent and convenient.

/// Push `T` into `Self` in builder-style
pub trait PushOwned<T> {
    fn push(self, val: T) -> Self;

    fn try_push(self, val: Option<T>) -> Self
    where
        Self: Sized,
    {
        if let Some(val) = val {
            self.push(val)
        } else {
            self
        }
    }
}

/// Set `T` in `Self` in builder-style
pub trait SetOwned<T> {
    fn set(self, val: T) -> Self;

    fn try_set(self, val: Option<T>) -> Self
    where
        Self: Sized,
    {
        if let Some(val) = val {
            self.set(val)
        } else {
            self
        }
    }
}

/// Trait provide helper methods that makes working with `Self` kinda
/// declarative
pub trait DeclarativeConfig: Sized {
    /// This method accept closure that configure `self`
    ///
    /// ```no_test
    /// Foo::default()
    ///     .config(|conf| {
    ///         // config foo in this closure
    ///     });
    /// ```
    fn config(self, block: impl FnOnce(Self) -> Self) -> Self {
        block(self)
    }

    /// Same as `config` but will be called if `condition` is `true`
    fn config_if(self, condition: bool, block: impl FnOnce(Self) -> Self) -> Self {
        if condition {
            self.config(block)
        } else {
            self
        }
    }

    /// if `condition` is `true` then `block` will be called, otherwise
    /// `else_blcok` will be called
    fn config_if_else(
        self,
        condition: bool,
        block: impl FnOnce(Self) -> Self,
        else_block: impl FnOnce(Self) -> Self,
    ) -> Self {
        if condition {
            self.config(block)
        } else {
            self.config(else_block)
        }
    }
}
