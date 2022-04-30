pub mod m
{
 use retrieve::*;
 #[pub_mod_pub_use(submodule_for_the_test)]
 #[test]
 fn test_in_situ()
 {
  // explicit sub-module namespace(mod + use)
  assert_eq!(submodule_for_the_test::VALUE, 123usize);

  // implicit sub-module namespace(pub mod)
  assert_eq!(VALUE, 123usize);
 }
}

#[test]
fn test_exo_situ()
{
 // access from the super-module(pub mod + use)
 assert_eq!(m::submodule_for_the_test::VALUE, 123usize);

 // access from the super-module(pub mod + pub use)
 assert_eq!(m::VALUE, 123usize);
}
