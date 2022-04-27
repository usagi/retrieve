use retrieve::*;

#[mod_pub_use(x)]
fn main()
{
 let x = X {
  a: 1,
  b: 2
 };

 println!("{:?}", x.a() + x.b());
}
