# Savory
**Rust / Wasm frontend library for building user interfaces.**

[![master docs](https://img.shields.io/badge/docs-master-blue.svg)](https://malrusayni.gitlab.io/savory/savory/)
&middot;
[![crate info](https://img.shields.io/crates/v/savory.svg)](https://crates.io/crates/savory)
&middot;
[![pipeline](https://gitlab.com/MAlrusayni/savory/badges/master/pipeline.svg)](https://gitlab.com/MAlrusayni/savory/pipelines)
&middot;
[![rustc version](https://img.shields.io/badge/rustc-stable-green.svg)](https://crates.io/crates/savory)
&middot;
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Savory is library for building user interface based on Seed

[Documentation](https://malrusayni.gitlab.io/savory/savory_core/)

# Features

- **Views**: Views can be any type implement `View` trait or any standalone
  function that returns `Node`, views can be trait object which make them very
  composable.
- **Elements**: Savory uses elements as core building unit when building
  stateful UI. Elements owns thier state and handle user inputs via messages.
- **Collection of UI elements**: Savory ships with collection of resuable and
  themeable UI elements.
- **Theme**: UI elements can be themed by any type that implement `ThemeImpl`
  trait, themes have full control on the element appearance.
- **Typed HTML**: Use typed CSS and HTML attributes, Savory try hard not to rely
  on strings when creating CSS and HTML attributes since these can produce hard
  to debug bugs.
- **Enhance Seed API**: Enhancement on Seed API that makes working with `Node`,
  `Orders` fun.

Savory tries to make writing UI elements fun and boilerplate free.

Savory crates:
- `savory`: savory CLI
- [`savory_core`]: Library for building user interface (this crate)
- [`savory_html`]: Typed HTML for Savory
- [`savory_elements`]: UI Elements based on Savory
- [`savory_derive`]: Helper derives

[`savory_core`]: https://gitlab.com/MAlrusayni/savory/tree/master/core
[`savory_html`]: https://gitlab.com/MAlrusayni/savory/tree/master/html
[`savory_elements`]: https://gitlab.com/MAlrusayni/savory/tree/master/elements
[`savory_derive`]: https://gitlab.com/MAlrusayni/savory/tree/master

# Examples

TODO: for now you can look at examples folder in each crate

# Qucikstart

TODO

# Screenshot

![Screenshot](screenshot.png)

# License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Savory by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
