use savory_core::prelude::View;
use std::rc::Rc;

/// Trait that makes the view more resuable by accepting `Self::Style`
///
/// Reusable view should implement this trait.
pub trait StyledView: View + Stylable {
    /// view with the passed styled
    fn styled_view(&self, style: Self::Style) -> Self::Output;
}

/// Trait used to define view style and styler with getter functions
pub trait Stylable {
    /// Style used by the view
    type Style;
    /// Styler that generate Style according to the view data
    type Styler;

    fn styler(&self) -> Self::Styler;
    fn style(&self) -> Self::Style;
}

/// Type that hold function which takes `&E` and return `Style`, this function
/// is used every time when elemet/view get viewed.
///
/// Usally `&E` is reference to type `<Element as ThemeLens>::Lens` or element
/// type.
pub struct Styler<E, S>(Rc<dyn Fn(&E) -> S>);

impl<E, S> Styler<E, S> {
    pub fn new(styler: impl Fn(&E) -> S + 'static) -> Self {
        Styler(Rc::new(styler))
    }

    pub fn get(&self, e: &E) -> S {
        self.0(e)
    }
}

impl<E, S, T> From<T> for Styler<E, S>
where
    T: Fn(&E) -> S + 'static,
{
    fn from(val: T) -> Self {
        Self(Rc::new(val))
    }
}

impl<E, S> From<Rc<dyn Fn(&E) -> S>> for Styler<E, S> {
    fn from(val: Rc<dyn Fn(&E) -> S>) -> Self {
        Self(val)
    }
}

impl<E, S> From<Rc<Styler<E, S>>> for Styler<E, S> {
    fn from(val: Rc<Styler<E, S>>) -> Self {
        Self(Rc::clone(&val.0))
    }
}

impl<E, S> Clone for Styler<E, S> {
    fn clone(&self) -> Self {
        Styler(Rc::clone(&self.0))
    }
}

impl<E, S: Default> Default for Styler<E, S> {
    fn default() -> Self {
        Self(Rc::new(|_| S::default()))
    }
}
