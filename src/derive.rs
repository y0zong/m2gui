use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn m2_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    if input.sig.ident != "main" {
        panic!("`m2_main` can only be used on a function called 'main'.")
    }

    TokenStream::from(quote! {
        #[no_mangle]
        #[cfg(target_os = "android")]
        use ndk_glue;
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity: *mut std::os::raw::c_void,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize,
        ) {
            ndk_glue::init(
                activity as _,
                saved_state as _,
                saved_state_size as _,
                main,
            );
        }

        #[no_mangle]
        #[cfg(target_os = "ios")]
        extern "C" fn main_rs() {
            main();
        }

        #[allow(unused)]
        #input
    })
}