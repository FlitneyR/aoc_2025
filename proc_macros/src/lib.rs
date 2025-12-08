extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(FromRegexCaptures)]
pub fn derive_from_regex_captures(ts: TokenStream) -> TokenStream {
    // let syn parse the input stream
    let input = syn::parse_macro_input!(ts as syn::DeriveInput);

    // the type we're deriving the trait for
    let struct_type = input.ident;

    match &input.data {
        // we're only deriving this trait for structs of named fields
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named( fields ), .. }) => {
            let mut members = quote::quote! {};

            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap();
                members.extend(quote::quote! {
                    #field_name: captures.name(stringify!(#field_name))
                        .ok_or(FromRegexCapturesError::MissingField(stringify!(#field_name)))?
                        .as_str().parse()
                        .map_err(|e| FromRegexCapturesError::FailedToParse(stringify!(#field_name)))?,
                });
            }

            quote::quote! {
                impl aoc_2025_common::FromRegexCaptures for #struct_type {
                    fn from_regex_captures(captures: &regex::Captures) -> Result<Self, aoc_2025_common::FromRegexCapturesError> {
                        use aoc_2025_common::FromRegexCapturesError;
                        Ok(Self { #members })
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}
