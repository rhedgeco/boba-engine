use darling::{ast::NestedMeta, util::PathList, Error, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, TokenStreamExt};
use syn::{parse_macro_input, spanned::Spanned, ItemStruct};

#[derive(FromMeta)]
struct PearlArgs {
    #[darling(rename = "listen")]
    listeners: Option<PathList>,
}

#[proc_macro_attribute]
pub fn pearl(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let ident = &item.ident;
    let (implgen, typegen, wheregen) = item.generics.split_for_impl();

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(meta) => meta,
        Err(e) => return TokenStream::from(Error::from(e).write_errors()),
    };

    let pearl_args = match PearlArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let mut registers = proc_macro2::TokenStream::new();
    if let Some(listeners) = pearl_args.listeners {
        for path in listeners.iter() {
            registers.append_all(quote_spanned! { path.span() =>
                register.event::<#path>();
            })
        }
    }

    let output = quote! {
        #item

        impl #implgen ::boba_core::Pearl for #ident #typegen #wheregen {
            fn register(register: &mut impl ::boba_core::EventRegister<Self>) {
                #registers
            }
        }
    };

    output.into()
}
