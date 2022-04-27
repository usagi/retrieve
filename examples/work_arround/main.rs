use retrieve::*;

#[mod_pub_use(x)]
fn main()
{
 let x = X::new(1);
 println!("{:?} {:?}", x.a(), x.to_string());
}
