use retrieve::*;

#[mod_pub_use(display)] // <-- this attribute cling to the use tokens.
use super::X; // <-- attribute style proc-macro are need some of valid tokens.
              // and you can use the simplest fake tokens if you need sometimes.
              // type __ = ();
