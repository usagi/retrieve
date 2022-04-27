use super::X;
use std::fmt::{
 Display,
 Formatter,
 Result
};

impl Display for X
{
 fn fmt(&self, f: &mut Formatter<'_>) -> Result
 {
  match self.a
  {
   1 => f.write_str("One!"),
   _ => f.write_str("Not one!")
  }
 }
}
