#![recursion_limit = "1024"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate edit_distance as ed;

use proc_macro::TokenStream;
use syn::Ident;

use std::iter::once;

fn get_op_arity(tra1t: &str) -> Option<usize> {
    match tra1t {
        "Quasigroup" | "Monoid" | "Semigroup" | "Loop" | "Group" | "GroupAbelian" => Some(1),
        "Ring" | "RingCommutative" | "Field" => Some(2),
        _ => None,
    }
}

fn get_closest_trait(tra1t: &str) -> &str {
    ["Quasigroup", "Monoid", "Semigroup", "Group", "GroupAbelian", "Ring", "RingCommutative", "Field"].iter()
        .map(|t| (ed::edit_distance(t, tra1t), t))
        .min().expect("Hardcoded array which we are iterating should never be empty. Programming error.").1
}

fn get_dependencies(tra1t: &str, op: usize) -> Vec<String> {
    match tra1t {
        "Quasigroup" => vec![],
        "Monoid" => vec!["Quasigroup"],
        "Semigroup" => vec![],
        "Loop" => vec!["Semigroup"],
        "Group" => vec!["Monoid", "Quasigroup", "Loop", "Semigroup"],
        "GroupAbelian" => vec!["Group", "Monoid", "Quasigroup", "Loop", "Semigroup"],
        _ => match tra1t {
            "Ring" => if op == 0 {
                vec!["GroupAbelian", "Group", "Monoid", "Quasigroup", "Loop", "Semigroup"]
            } else {
                vec!["Monoid", "Quasigroup", "Loop", "Semigroup"]
            },
            "RingCommutative" => if op == 0 {
                vec!["Ring", "GroupAbelian", "Group", "Monoid", "Quasigroup", "Loop", "Semigroup"]
            } else {
                vec!["Ring", "Monoid", "Quasigroup", "Loop", "Semigroup"]
            },
            "Field" => if op == 0 {
                vec!["RingCommutative", "Ring", "GroupAbelian", "Group", "Monoid", "Quasigroup", "Loop", "Semigroup"]
            } else {
                vec!["GroupAbelian", "Group", "Monoid", "Quasigroup", "Loop", "Semigroup"]
            },
            _ => panic!("Unknown Alga trait `{}`. Programming error.", tra1t),
        }
    }.into_iter().map(String::from).collect()
}

#[proc_macro_derive(Alga, attributes(alga_traits))]
pub fn derive_alga(input: TokenStream) -> TokenStream {
    use syn::MetaItem::*;
    use syn::NestedMetaItem::*;
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
            if let &List(ref name, ref value) = n {
                (name, value)
            } else {
                panic!("Operator has to be provided via #[alga_traits(Trait(Op1, ..))]");
            }
        }).flat_map(|(name, value)| {
            let name = name.as_ref();
            let arity = get_op_arity(name)
                .expect(&format!("Invalid Alga trait provided. Did you mean `{}`?", get_closest_trait(name)));
            assert!(value.len() == arity, "Exactly {} operators need to be specified.", arity);
            let iter = once(name.into()).chain(
                get_dependencies(name, 0)
                    .into_iter()
            ).map(|n| {
                let arity = get_op_arity(&n)
                    .expect(&format!("Invalid Alga trait `{}`. Programming error.", n));
                (Ident::from(format!("Abstract{}", n)), if arity == 1 {
                    vec![value[0].clone()]
                } else {
                    value.clone()
                })
            });
            if arity == 1 {
                iter.collect::<Vec<_>>()
            } else {
                iter.chain(
                    get_dependencies(name, 1)
                        .into_iter()
                        .map(|n| {
                            let arity = get_op_arity(&n)
                                .expect(&format!("Invalid Alga trait `{}`. Programming error.", n));
                            (Ident::from(format!("Abstract{}", n)), if arity == 1 {
                                vec![value[1].clone()]
                            } else {
                                value.clone()
                            })
                        }
                )).collect()
            }
        })
        .unzip();
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
            impl #impl_generics _alga::general::#tra1t<#(#op,)*> for #name #ty_generics #where_clause {})*
        };
    }).parse().unwrap()
}
