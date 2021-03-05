use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

// The "syn" crate parses Rust code from a string into a data structure that we can perform
// operations on.
use syn;

// The "quote" crate turns "syn" data structures back into Rust code.
use quote::quote;

// "hello_macro_derive" function will be called when a user specifies #[derive(HelloMacro] on a
// type.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    // from token to ast.
    let ast = syn::parse2(input).unwrap_or_else(|e| panic!("{}", e));

    // Build the trait implementation.
    let output = impl_hello_macro(&ast);

    TokenStream::from(output)
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
