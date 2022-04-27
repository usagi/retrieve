use super::X;

pub trait B
{
 fn b(&self) -> i32;
}

impl B for X
{
 fn b(&self) -> i32
 {
  self.b
 }
}
