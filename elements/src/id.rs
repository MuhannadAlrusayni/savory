use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Display, Clone, PartialEq, PartialOrd, Hash, From)]
pub struct Id(String);

impl Id {
    pub fn new(id: impl ToString) -> Id {
        Id(id.to_string())
    }

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
