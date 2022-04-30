mod m
{
 use retrieve::*;
 #[mod_use(submodule_for_the_test)]
 #[test]
 fn test_in_situ()
 {
  // explicit sub-module namespace(mod + use)
  assert_eq!(submodule_for_the_test::VALUE, 123usize);
  // implicit sub-module namespace(pub mod)
  assert_eq!(VALUE, 123usize);
 }
}

// The `test_exo_situ` should be compile error, but it is not an error for the `mod_use` pattern.
// (Here I should originally test for a compile error, but the implementation is too cumbersome, it omitted.)
// #[test]
// fn test_exo_situ()
// {
//  // access from the super-module(pub mod + use)
//  assert_eq!(m::submodule_for_the_test::VALUE, 123usize);
//
//  // access from the super-module(pub mod + pub use)
//  assert_eq!(m::VALUE, 123usize);
// }
