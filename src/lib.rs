#![feature(decl_macro)]

pub mod el;
// pub mod layout;
pub mod macros;
pub mod model;
pub mod properties;
pub mod routable;
pub mod theme;
pub mod view;

#[macro_use]
extern crate seed;

#[macro_use]
extern crate derive_more;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
