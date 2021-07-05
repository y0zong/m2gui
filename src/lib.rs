extern crate proc_macro;

mod app;
mod derive;

use proc_macro::TokenStream;
#[proc_macro_attribute]
pub fn m2_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive::m2_main(attr, item)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
