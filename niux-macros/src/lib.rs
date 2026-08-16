use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(replace_env, attributes(replace_env))]
pub fn replace_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        _ => {
            return syn::Error::new(name.span(), "replace_env can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let punctuated = match fields {
        syn::Fields::Named(fields) => fields.named,
        _ => {
            return syn::Error::new(
                name.span(),
                "replace_env can only be derived for structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut type_fields = Vec::new();

    for field in punctuated {
        let mut skip = false;

        for attr in field.attrs {
            if attr.path().is_ident("replace_env")
                && let Ok(arg) = attr.parse_args::<syn::Ident>()
                && arg == "skip"
            {
                skip = true;
            }
        }
        if !skip && let Some(ident) = &field.ident {
            type_fields.push(ident.clone());
        }
    }

    let generated: Vec<_> = type_fields
        .iter()
        .map(|f| {
            quote! {
                self.#f.replace_env();
            }
        })
        .collect();

    quote! {
        impl crate::utils::replace_env::ReplaceEnv for #name {
            fn replace_env(&mut self) {
                use crate::utils::replace_env::ReplaceEnv;
                #(#generated)*
            }
        }
    }
    .into()
}
