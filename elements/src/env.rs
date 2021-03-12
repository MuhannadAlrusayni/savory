use crate::prelude::DesignSystem;
use savory::prelude::Env;

pub trait EnvExt {
    /// return design system from the environment
    ///
    /// # panic
    /// panic if design system isn't found
    fn ds(&self) -> DesignSystem;
}

impl EnvExt for Env {
    fn ds(&self) -> DesignSystem {
        self.get::<DesignSystem>()
            .expect("Design System isn't found in the environment")
    }
}
