use retrieve::*;

/// Example of in the below:
/// ```
/// #[mod_pub_use(a, b)]
/// ```
/// will be the same as:
/// ```
/// mod a;
/// pub use a::*;
/// mod b;
/// pub use b::*;
/// ```

#[mod_pub_use(a, b)]
pub struct X
{
 pub a: i32,
 pub b: i32
}
