#![recursion_limit = "1024"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate edit_distance as ed;

use proc_macro::TokenStream;
use syn::Ident;

use std::iter::once;

fn get_op_arity(tra1t: &str) -> usize {
    match tra1t {
        "Quasigroup" | "Monoid" | "Semigroup" | "Loop" | "Group" | "GroupAbelian" => 1,
        "Ring" | "RingCommutative" | "Field" => 2,
        _ => panic!("Invalid Alga trait provided. Did you mean `{}`?", get_closest_trait(tra1t)),
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
    use syn::Lit::*;

    let item = syn::parse_derive_input(&input.to_string()).unwrap();
    let name = &item.ident;
    let (i, t, w) = item.generics.split_for_impl();
    let (impl_generics, ty_generics) = (once(&i).cycle(), once(&t).cycle());

    let iter = item.attrs.iter()
        .filter_map(|a|
            if let List(ref ident, ref traits) = a.value {
                Some((ident, traits))
            } else {
                None
            })
        .filter(|&(i, _)| i == "alga_traits")
        .flat_map(|(_, v)| v)
        .map(|t|
            match t {
                &MetaItem(List(ref name, ref value)) => (name.as_ref(), value.clone()),
                &MetaItem(NameValue(ref name, ref value)) => if name == "Where" {
                    (name.as_ref(), vec![Literal(value.clone())])
                } else {
                    panic!("Where clause should be defined with `Where = \"TypeParameter: Trait\"`.");
                },
                _ => panic!("Operator has to be provided via #[alga_traits(Trait(Operators))]"),
            }
        );
    let mut traits: Vec<(_, _, Option<_>)> = vec![];
    let mut valid_clause_place = false;
    let mut first = true;
    for (name, value) in iter {
        if name == "Where" {
            if valid_clause_place {
                let len = traits.len();
                if let Literal(Str(ref clause, syn::StrStyle::Cooked)) = value[0] {
                    let mut clause = syn::parse_where_clause(&format!("where {}", clause))
                                .expect("Where clauses bound was invalid.");
                    clause.predicates.extend(w.predicates.clone());
                    traits[len - 1].2 = Some(clause);
                }else {
                    panic!("Where clause should be a string literal.");
                }
                valid_clause_place = false;
            } else {
                if first {
                    panic!("There is where clause before any traits to apply it to.");
                } else {
                    panic!("There is multiple where clauses next to each other.");
                }
            }
        } else {
            first = false;
            valid_clause_place = true;
            traits.push((name, value, None));
        }
    }
    let (tra1t, op, where_clause) = traits.into_iter()
        .flat_map(|(name, value, clause)| {
            let arity = get_op_arity(name);
            let value = value.clone();
            assert!(value.len() == arity, "Exactly {} operators need to be specified.", arity);
            let create_tuple = |n: &str, i: usize| {
                let value = if get_op_arity(n) == 1 {
                    vec![value[i].clone()]
                } else {
                    value.clone()
                };
                (Ident::from(format!("Abstract{}", n)), value, clause.clone())
            };
            let create_tuple = &create_tuple;
            let iter = once(name.into())
                .chain(get_dependencies(name, 0))
                .map(|n| create_tuple(&n, 0));
            if arity == 1 {
                iter.collect::<Vec<_>>()
            } else {
                iter.chain(
                    get_dependencies(name, 1)
                        .into_iter()
                        .map(|n| create_tuple(&n, 1))
                ).collect()
            }
        })
        .unzip3();
    assert!(!tra1t.is_empty(),
    "Atleast one trait is required to be implemented.\n         Trait can be specified with `#[alga_traits(Trait(Operators))]` attribute.");

    let dummy_const = Ident::new(format!("_ALGA_DERIVE_{}", name));
    let name = once(&name).cycle();
    quote!(
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate alga as _alga;
            #(
                #[automatically_derived]
                impl #impl_generics _alga::general::#tra1t<#(#op,)*> for #name #ty_generics #where_clause {}
            )*
        };
    ).parse().unwrap()
}

trait Unzip3<A, B, C> {
    fn unzip3(self) -> (Vec<A>, Vec<B>, Vec<C>);
}

impl<A, B, C, I> Unzip3<A, B, C> for I
    where I: Iterator<Item=(A, B, C)>,
{
    fn unzip3(self) -> (Vec<A>, Vec<B>, Vec<C>) {
        let hint = self.size_hint().1.unwrap_or(Vec::<A>::new().capacity());
        let (mut va, mut vb, mut vc) = (Vec::with_capacity(hint), Vec::with_capacity(hint), Vec::with_capacity(hint));
        for (a, b, c) in self {
            va.push(a);
            vb.push(b);
            vc.push(c);
        }
        (va, vb, vc)
    }
}
