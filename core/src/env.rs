use std::{cell::RefCell, rc::Rc};
use type_map::TypeMap;

type Branch = Rc<RefCell<TypeMap>>;

#[derive(Clone)]
pub struct Env {
    branch: Branch,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn base_branch() -> Self {
        Env {
            branch: Rc::default(),
            parent: None,
        }
    }

    pub fn branch(&self) -> Self {
        let mut res = Self::base_branch();
        res.parent = Some(Box::new(self.clone()));
        res
    }

    pub fn share(&self) -> Self {
        self.clone()
    }

    pub fn get<T: Clone + 'static>(&self) -> Option<T> {
        self.branch
            .borrow()
            .get::<T>()
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|env| env.get::<T>()))
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.branch.borrow().contains::<T>()
    }

    /// Insert a value in to the environment.
    ///
    /// # Accessibility
    ///
    /// This value is only accessible in the current environment branch and
    /// descendants branchs, predecessors branches cannot access this value and
    /// they don't know that it's even exists.
    ///
    /// this method won't overwrite existing values in predecessors branches, it
    /// will only shadow them!. In case this value type already exists in the
    /// current environment it will be replaced.
    pub fn insert<T: 'static>(self, val: T) -> Self {
        self.branch.borrow_mut().insert(val);
        self
    }

    /// Set a new variable in current environment branch without panicing.
    ///
    /// Same as `set()` but it won't panic, it will set nothing if type value
    /// already exists.
    pub fn try_insert<T: 'static>(self, val: T) -> Self {
        if !self.contains::<T>() {
            self.branch.borrow_mut().insert(val);
        }
        self
    }

    /// Update environment variable
    ///
    /// # panic
    ///
    /// this method will panic if the passed type value is not initialized, you
    /// can use `try_update` for non-panicing version.
    pub fn update<T: 'static, F: FnOnce(T) -> T>(self, f: F) -> Self {
        // update current environment
        let val = { self.branch.borrow_mut().remove::<T>() };
        if let Some(val) = val {
            self.branch.borrow_mut().insert(f(val));
            return self;
        }

        // panic if type value doesn't exists
        panic!(
            "Env::update(..) failed, type {} doesn't exists in current environment branch",
            std::any::type_name::<T>()
        )
    }

    pub fn try_update<T: 'static, F: FnOnce(T) -> T>(self, f: F) -> Self {
        // update current environment
        let val = { self.branch.borrow_mut().remove::<T>() };
        if let Some(val) = val {
            self.branch.borrow_mut().insert(f(val));
            return self;
        }
        self
    }
}

// FIXME: test all use cases for Env
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        #[derive(Clone, Eq, PartialEq, Debug)]
        struct DarkTheme(bool);

        let base = Env::base_branch()
            .insert("Hi There".to_owned())
            .insert(DarkTheme(true));
        assert_eq!(base.get::<DarkTheme>(), Some(DarkTheme(true)));
        assert_eq!(base.get::<String>(), Some("Hi There".to_owned()));

        let branch = base.branch().insert(DarkTheme(false));

        assert_eq!(base.get::<DarkTheme>(), Some(DarkTheme(true)));
        assert_eq!(base.get::<String>(), Some("Hi There".to_owned()));
        assert_eq!(branch.get::<DarkTheme>(), Some(DarkTheme(false)));
        assert_eq!(branch.get::<String>(), Some("Hi There".to_owned()));

        let branch = branch.update(|_: DarkTheme| DarkTheme(true));

        assert_eq!(base.get::<DarkTheme>(), Some(DarkTheme(true)));
        assert_eq!(base.get::<String>(), Some("Hi There".to_owned()));
        assert_eq!(branch.get::<DarkTheme>(), Some(DarkTheme(true)));
        assert_eq!(branch.get::<String>(), Some("Hi There".to_owned()));

        let base_2 = base.share().update(|_: DarkTheme| DarkTheme(false));

        assert_eq!(base.get::<DarkTheme>(), Some(DarkTheme(false)));
        assert_eq!(base.get::<String>(), Some("Hi There".to_owned()));
        assert_eq!(base_2.get::<DarkTheme>(), Some(DarkTheme(false)));
        assert_eq!(base_2.get::<String>(), Some("Hi There".to_owned()));
    }
}
