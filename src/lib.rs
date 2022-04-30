// extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
 self,
 parse_macro_input,
 AttributeArgs
};

/// Example to use:
/// ```
/// #[mod_use(a,b,c)]
/// ```
/// And then it will be the same as:
/// ```
/// mod a;
/// use a::*;
/// mod b;
/// use b::*;
/// mod c;
/// use c::*;
/// ```
#[proc_macro_attribute]
pub fn mod_use(args: TokenStream, input: TokenStream) -> TokenStream
{
 let args = parse_macro_input!(args as AttributeArgs);
 let tss = args
  .into_iter()
  .filter_map(|arg| {
   match arg
   {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
    {
     path
      .get_ident()
      .map(|module_name| format!("mod {module_name}; use {module_name}::*;", module_name = module_name))
    },
    _ => None
   }
  })
  .chain(std::iter::once(input.to_string()))
  .collect::<Vec<_>>();
 let r = tss.join("\n");
 if cfg!(feature = "print")
 {
  eprintln!("[(proc-macro)retrieve::mod_use+print]\n{}", r);
 }
 r.parse::<TokenStream>().expect("TokenStream parse error.")
}

/// Example to use:
/// ```
/// #[pub_mod_use(a,b,c)]
/// ```
/// And then it will be the same as:
/// ```
/// pub mod a;
/// use a::*;
/// pub mod b;
/// use b::*;
/// pub mod c;
/// use c::*;
/// ```
#[proc_macro_attribute]
pub fn pub_mod_use(args: TokenStream, input: TokenStream) -> TokenStream
{
 let args = parse_macro_input!(args as AttributeArgs);
 let tss = args
  .into_iter()
  .filter_map(|arg| {
   match arg
   {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
    {
     path
      .get_ident()
      .map(|module_name| format!("pub mod {module_name}; use {module_name}::*;", module_name = module_name))
    },
    _ => None
   }
  })
  .chain(std::iter::once(input.to_string()))
  .collect::<Vec<_>>();
 let r = tss.join("\n");
 if cfg!(feature = "print")
 {
  eprintln!("[(proc-macro)retrieve::pub_mod_use+print]\n{}", r);
 }
 r.parse::<TokenStream>().expect("TokenStream parse error.")
}

/// Example to use:
/// ```
/// #[mod_pub_use(a,b,c)]
/// ```
/// And then it will be the same as:
/// ```
/// mod a;
/// pub use a::*;
/// mod b;
/// pub use b::*;
/// mod c;
/// pub use c::*;
/// ```
#[proc_macro_attribute]
pub fn mod_pub_use(args: TokenStream, input: TokenStream) -> TokenStream
{
 let args = parse_macro_input!(args as AttributeArgs);
 let tss = args
  .into_iter()
  .filter_map(|arg| {
   match arg
   {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
    {
     path
      .get_ident()
      .map(|module_name| format!("mod {module_name}; pub use {module_name}::*;", module_name = module_name))
    },
    _ => None
   }
  })
  .chain(std::iter::once(input.to_string()))
  .collect::<Vec<_>>();
 let r = tss.join("\n");
 if cfg!(feature = "print")
 {
  eprintln!("[(proc-macro)retrieve::mod_pub_use+print]\n{}", r);
 }
 r.parse::<TokenStream>().expect("TokenStream parse error.")
}

/// Example to use:
/// ```
/// #[pub_mod_pub_use(a,b,c)]
/// ```
/// And then it will be the same as:
/// ```
/// pub mod a;
/// pub use a::*;
/// pub mod b;
/// pub use b::*;
/// pub mod c;
/// pub use c::*;
/// ```
#[proc_macro_attribute]
pub fn pub_mod_pub_use(args: TokenStream, input: TokenStream) -> TokenStream
{
 let args = parse_macro_input!(args as AttributeArgs);
 let tss = args
  .into_iter()
  .filter_map(|arg| {
   match arg
   {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
    {
     path
      .get_ident()
      .map(|module_name| format!("pub mod {module_name}; pub use {module_name}::*;", module_name = module_name))
    },
    _ => None
   }
  })
  .chain(std::iter::once(input.to_string()))
  .collect::<Vec<_>>();
 let r = tss.join("\n");
 if cfg!(feature = "print")
 {
  eprintln!("[(proc-macro)retrieve::pub_mod_pub_use+print]\n{}", r);
 }
 r.parse::<TokenStream>().expect("TokenStream parse error.")
}

/// Example to use:
/// ```
/// #[pub_mod(a,b,c)]
/// ```
/// And then it will be the same as:
/// ```
/// pub mod a;
/// pub mod b;
/// pub mod c;
/// ```
/// Note: This is an extra syntax sugar for *mod_*use series  users. Because I think unification of syntax is so beautiful. I author not recommended to use only *mod without *mod_*use series.
#[proc_macro_attribute]
pub fn pub_mod(args: TokenStream, input: TokenStream) -> TokenStream
{
 let args = parse_macro_input!(args as AttributeArgs);
 let tss = args
  .into_iter()
  .filter_map(|arg| {
   match arg
   {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
    {
     path
      .get_ident()
      .map(|module_name| format!("pub mod {module_name};", module_name = module_name))
    },
    _ => None
   }
  })
  .chain(std::iter::once(input.to_string()))
  .collect::<Vec<_>>();
 let r = tss.join("\n");
 if cfg!(feature = "print")
 {
  eprintln!("[(proc-macro)retrieve::pub_mod+print]\n{}", r);
 }
 r.parse::<TokenStream>().expect("TokenStream parse error.")
}

/// Example to use:
/// ```
/// #[r#mod(a,b,c)]
/// ```
/// And then it will be the same as:
/// ```
/// mod a;
/// mod b;
/// mod c;
/// ```
/// Note: This is an extra syntax sugar for *mod_*use series  users. Because I think unification of syntax is so beautiful. I author not recommended to use only *mod without *mod_*use series.
#[proc_macro_attribute]
pub fn r#mod(args: TokenStream, input: TokenStream) -> TokenStream
{
 let args = parse_macro_input!(args as AttributeArgs);
 let tss = args
  .into_iter()
  .filter_map(|arg| {
   match arg
   {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) =>
    {
     path
      .get_ident()
      .map(|module_name| format!("mod {module_name};", module_name = module_name))
    },
    _ => None
   }
  })
  .chain(std::iter::once(input.to_string()))
  .collect::<Vec<_>>();
 let r = tss.join("\n");
 if cfg!(feature = "print")
 {
  eprintln!("[(proc-macro)retrieve::r#mod+print]\n{}", r);
 }
 r.parse::<TokenStream>().expect("TokenStream parse error.")
}
