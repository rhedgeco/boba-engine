use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    // parse the input
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);

    // gather generics for quote
    let (implgen, typegen, wheregen) = generics.split_for_impl();

    // create output
    let output = quote! {
        unsafe impl #implgen ::boba_ecs::Component for #ident #typegen #wheregen {
            const COMPONENT_ID: ::boba_ecs::ComponentId = ::boba_ecs::ComponentId::hash_str(
                concat!(
                    module_path!(), "::", stringify!(#ident),
                    ":", file!(), ":", line!(), ":", column!()
                )
            );
        }
    };

    // convert output and return
    output.into()
}
