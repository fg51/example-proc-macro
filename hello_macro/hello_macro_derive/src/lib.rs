use proc_macro::TokenStream;

// The "syn" crate parses Rust code from a string into a data structure that we can perform
// operations on.
use syn;

// The "quote" crate turns "syn" data structures back into Rust code.
use quote::quote;

// "hello_macro_derive" function will be called when a user specifies #[derive(HelloMacro] on a
// type.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap_or_else(|e| panic!("{}", e)); // from input to ast.

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
