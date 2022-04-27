use super::X;

pub trait A
{
 fn a(&self) -> i32;
}

impl A for X
{
 fn a(&self) -> i32
 {
  self.a
 }
}
