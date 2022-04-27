[![github]](https://github.com/usagi/retrieve)&ensp;[![crates-io]](https://crates.io/crates/retrieve)&ensp;[![docs-rs]](https://docs.rs/retrieve)<br>
[![Build Status](https://travis-ci.org/usagi/retrieve.svg?branch=master)](https://travis-ci.org/usagi/retrieve)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# retrieve

`mod x` and `use x::*` pattern syntax sugar of the proc-macro attribute.

- `retrieve::`
  - `mod_use` => `mod x; use x::*;` for a module internal.
  - `pub_mod_use` => `pub mod x; use x::*` for a public nested modules.
  - `mod_pub_use` => `mod x; pub use x::*` for a public flatten modules with separated source writting.
  - `pub_mod_pub_use` => `pub x; pub use x::x` for a public nested modules with flatten alias in the root.

I am tired of writing the "xxx" pattern over and over again for structured beatiful source code!😝 And I like the stylish attribute proc-macro style syntax sugars.💖

## Example: [examples/mod_pub_use/](examples/mod_pub_use/)

- src/
  - main.rs
  - x.rs ; Or it can move x/mod.rs if you like module-name/mod.rs style.
  - x/
    - a.rs ; `crate::x::a::*`
    - b.rs ; `crate::x::b::*`

1. main.rs:

```rust
use retrieve::*;

#[mod_pub_use(x)] // <-- here!; it's the same as `mod x; pub use x::*;`
fn main()
{
 // `X` from x::X
 let x = X {
  a: 1,
  b: 2
 };

 // And it from '2. x.rs'; `.a()` from the trait of `x::a::A` and `.b()` from the trait of `x::b::B`
 println!("{:?}", x.a() + x.b());
}
```

2. x.rs:

```rust
use retrieve::*;

#[mod_pub_use(a, b)] // <-- here!; it's the same as `mod a; pub use a::*;` and `mod b; pub use b::*;`
pub struct X
{
 pub a: i32,
 pub b: i32
}
```

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
