use crate::prelude::{DataLens, Design, Designer, ViewStyle};
use savory::prelude::Env;
use std::rc::Rc;

pub trait EnvExt {
    fn insert_designer<T>(self, designer: Rc<dyn Design<T>>) -> Self
    where
        T: DataLens + ViewStyle + 'static;

    fn try_insert_designer<T>(self, designer: Rc<dyn Design<T>>) -> Self
    where
        T: DataLens + ViewStyle + 'static;

    fn update_designer<T, F>(self, f: F) -> Self
    where
        T: DataLens + ViewStyle + 'static,
        F: FnOnce(Designer<T>) -> Rc<dyn Design<T>>;

    fn try_update_designer<T, F>(self, f: F) -> Self
    where
        T: DataLens + ViewStyle + 'static,
        F: FnOnce(Designer<T>) -> Rc<dyn Design<T>>;

    fn designer<T: 'static>(&self) -> Designer<T>;
}

impl EnvExt for Env {
    fn insert_designer<T>(self, designer: Rc<dyn Design<T>>) -> Self
    where
        T: DataLens + ViewStyle + 'static,
    {
        self.insert(Designer::from(designer))
    }

    fn try_insert_designer<T>(self, designer: Rc<dyn Design<T>>) -> Self
    where
        T: DataLens + ViewStyle + 'static,
    {
        self.try_insert(Designer::from(designer))
    }

    fn update_designer<T, F>(self, f: F) -> Self
    where
        T: DataLens + ViewStyle + 'static,
        F: FnOnce(Designer<T>) -> Rc<dyn Design<T>>,
    {
        self.update(|d: Designer<T>| Designer::from(f(d)))
    }

    fn try_update_designer<T, F>(self, f: F) -> Self
    where
        T: DataLens + ViewStyle + 'static,
        F: FnOnce(Designer<T>) -> Rc<dyn Design<T>>,
    {
        self.try_update(|d: Designer<T>| Designer::from(f(d)))
    }

    fn designer<T: 'static>(&self) -> Designer<T> {
        self.get::<Designer<T>>().expect(&format!(
            "{} isn't found in the environment",
            std::any::type_name::<Designer<T>>()
        ))
    }
}
