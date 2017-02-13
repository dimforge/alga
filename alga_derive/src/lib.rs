#![recursion_limit = "1024"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{Ident};

use std::iter::once;

#[proc_macro_derive(Alga, attributes(alga_traits))]
pub fn derive_alga(input: TokenStream) -> TokenStream {
    use syn::MetaItem::*;
    use syn::NestedMetaItem::*;
    use syn::Lit::*;
    use syn::StrStyle::*;
    let item = syn::parse_derive_input(&input.to_string()).unwrap();
    let name = &item.ident;
    let (tra1t, op): (Vec<_>, Vec<_>) = item.attrs.iter()
        .filter_map(|a| {
                if let List(ref ident, ref traits) = a.value {
                    Some((ident, traits))
                } else {
                    None
                }
            })
        .filter(|&(i, _)| i == "alga_traits")
        .flat_map(|(_, v)| v)
        .map(|t| {
            if let &MetaItem(ref n) = t {
                n
            } else {
                panic!("Instead of a literal, trait with operator expected:\n         #[alga_traits(Trait = Operator, ...)]");
            }
        }).map(|n| {
            if let &NameValue(ref name, ref value) = n {
                (name, value)
            } else {
                // TODO: Multioperator traits.
                panic!("Operator needs to be specified.");
            }
        }).map(|(name, value)| {
            if let &Str(ref value, Cooked) = value {
                // TODO: Error when trait given is not Alga trait.
                (Ident::from(format!("Abstract{}", name)), Ident::from(value.clone()))
            } else {
                panic!("Operator needs to be specified as string literal.");
            }
        }).unzip();
    assert!(!tra1t.is_empty(),
    "Atleast one trait is required to be implemented.\n         Trait can be specified with `#[alga_traits(Trait = Operator, ...)]` attribute.");

    let dummy_const = Ident::new(format!("_ALGA_DERIVE_{}", name));
    let name = once(&name).cycle();
    let (i, t, w) = item.generics.split_for_impl();
    let (impl_generics, ty_generics, where_clause) = (once(&i).cycle(), once(&t).cycle(), once(&w).cycle());
    (quote!{
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate alga as _alga;
            #(#[automatically_derived]
            impl #impl_generics _alga::general::#tra1t<#op> for #name #ty_generics #where_clause {})*
        };
    }).parse().unwrap()
}
