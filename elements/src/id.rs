use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Display, Clone, PartialEq, PartialOrd, Hash, From)]
pub struct Id(String);

/// Unique id used by elements and views
impl Id {
    /// Create a new Id from the passed `id`. Ids must be unique and used only
    /// by one element or view, when calling this function it's your
    /// responsibility to make sure this id is only assigned to one element or
    /// view.
    pub fn new(id: impl ToString) -> Id {
        Id(id.to_string())
    }

    /// Generate a unique Id
    ///
    /// Note: the generated Id is guaranteed to be unique, currently the
    /// generated id is incremental `u64`, but don't rely on this fact because
    /// this may change later.
    pub fn generate() -> Id {
        static CURRENT_ID: AtomicU64 = AtomicU64::new(0);
        let next = CURRENT_ID.fetch_add(1, Ordering::SeqCst).to_string();
        Id::new(next)
    }
}

impl From<&'static str> for Id {
    fn from(source: &'static str) -> Self {
        Id::new(source)
    }
}

impl From<Id> for String {
    fn from(source: Id) -> Self {
        source.0
    }
}

impl From<Id> for std::borrow::Cow<'static, str> {
    fn from(source: Id) -> Self {
        source.0.into()
    }
}
