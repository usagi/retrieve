use super::X;

pub trait New
{
 fn new(a: i32) -> X;
}

impl New for X
{
 fn new(a: i32) -> X
 {
  X {
   a
  }
 }
}
