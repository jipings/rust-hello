#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};
use quote::quote;

// 类函数宏(function-like macros) - custom!(...)
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// 属性宏(attribute macros) - #[CustomDerive]
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}


// 派生宏(derive macros) - #[derive(CustomDerive)]
#[proc_macro_derive(HlperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

// 属性宏(attribute macros) - #[CustomDerive]
/// This attribute macro will register and run a test 3 times, erroring only if all three times fail. 
/// Useful for situations when a test is flaky. 
#[proc_macro_attribute]
pub fn flaky_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = syn::parse_macro_input!(input as syn::ItemFn);
    let name = input_fn.sig.ident.clone();
    eprint!("name: {:#?}", name);

    TokenStream::from(quote! {
        #[test]
        fn #name() {
            #input_fn

            for i in 0..3 {
                println!("flaky_test retry {}", i);
                let r = std::panic::catch_unwind(|| {
                    #name();
                });
                if r.is_ok() {
                    return;
                }
                if i == 2 {
                    std::panic::resume_unwind(r.unwrap_err());
                }
            }
        }
    })
}