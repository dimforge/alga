//! # alga-derive
//!
//! Custom derive for `alga` traits.
//!
//! Supported traits:
//!
//! - `AbstractQuasigroup`
//! - `AbstractMonoid`
//! - `AbstractSemigroup`
//! - `AbstractGroup`
//! - `AbstractGroupAbelian`
//! - `AbstractRing`
//! - `AbstractRingCommutative`
//! - `AbstractField`
//!
//! ## Examples
//!
//! ~~~.ignore
//! extern crate alga;
//! #[macro_use]
//! extern crate alga_derive;
//!
//! use alga::general::Additive;
//!
//! #[derive(Alga)]
//! #[alga_traits(Group(Additive))]
//! struct Struct;
//! ~~~
//! This derive implements `AbstractGroup` marker trait with `Additive` operator and all
//! marker traits required by the algebraic groupness property
//! (`AbstractMonoid`, `AbstractSemigroup`, `AbstractLoop` and `AbstractQuasigroup`) for the target of the derive.
//!
//! Traits required by these marker traits (`Identity`, `PartialEq`, `TwoSidedInverse` and `AbstractMagma`) should be implemented manually.
//!
//! If `#[alga_quickcheck]` attribute is added for the target of the derive,
//! then `quickcheck` tests will be generated.
//! These tests will check that the algebraic properties of the derived trait are true for the type.
//! This attribute requires `quickcheck`s `Arbitrary` trait to be implemented for the target of the derive.
//!
//! ~~~.ignore
//! extern crate alga;
//! #[macro_use]
//! extern crate alga_derive;
//!
//! use alga::general::{Additive, AbstractGroup};
//!
//! #[derive(Alga)]
//! #[alga_traits(Group(Additive), Where = "T: AbstractGroup")]
//! #[alga_quickcheck(check(i32), check(i64))]
//! struct Struct<T>;
//! ~~~
//! When there is generic parameters on the target of the derive,
//! then all the concrete type parameters that the tests are generated for can be specified in
//! `alga_quickcheck` attribute by listing them in `check`s.
//!
//! If bounds are required for the `alga` traits to be implemented,
//! they can be listed by `Where = "A: Bound1. B: Bound2"`.

#![recursion_limit = "1024"]
extern crate edit_distance as ed;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Generics, Ident};

use std::iter::once;

fn get_op_arity(tra1t: &str) -> usize {
    match tra1t {
        "Quasigroup" | "Monoid" | "Semigroup" | "Loop" | "Group" | "GroupAbelian" => 1,
        "Ring" | "RingCommutative" | "Field" => 2,
        _ => panic!(
            "Invalid Alga trait provided. Did you mean `{}`?",
            get_closest_trait(tra1t)
        ),
    }
}

fn get_closest_trait(tra1t: &str) -> &str {
    [
        "Quasigroup",
        "Monoid",
        "Semigroup",
        "Group",
        "GroupAbelian",
        "Ring",
        "RingCommutative",
        "Field",
    ].iter()
        .map(|t| (ed::edit_distance(t, tra1t), t))
        .min()
        .expect("Hardcoded array which we are iterating should never be empty. Programming error.")
        .1
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
                vec![
                    "GroupAbelian",
                    "Group",
                    "Monoid",
                    "Quasigroup",
                    "Loop",
                    "Semigroup",
                ]
            } else {
                vec!["Monoid", "Quasigroup", "Loop", "Semigroup"]
            },
            "RingCommutative" => if op == 0 {
                vec![
                    "Ring",
                    "GroupAbelian",
                    "Group",
                    "Monoid",
                    "Quasigroup",
                    "Loop",
                    "Semigroup",
                ]
            } else {
                vec!["Ring", "Monoid", "Quasigroup", "Loop", "Semigroup"]
            },
            "Field" => if op == 0 {
                vec![
                    "RingCommutative",
                    "Ring",
                    "GroupAbelian",
                    "Group",
                    "Monoid",
                    "Quasigroup",
                    "Loop",
                    "Semigroup",
                ]
            } else {
                vec![
                    "GroupAbelian",
                    "Group",
                    "Monoid",
                    "Quasigroup",
                    "Loop",
                    "Semigroup",
                ]
            },
            _ => panic!("Unknown Alga trait `{}`. Programming error.", tra1t),
        },
    }.into_iter()
        .map(String::from)
        .collect()
}

fn get_props(tra1t: &str) -> Vec<(Ident, Ident, usize)> {
    match tra1t {
        "Quasigroup" => vec![("prop_inv_is_latin_square", 2)],
        "Monoid" => vec![("prop_operating_identity_element_is_noop", 1)],
        "Semigroup" => vec![("prop_is_associative", 3)],
        "GroupAbelian" => vec![("prop_is_commutative", 2)],
        "Ring" => vec![("prop_mul_and_add_are_distributive", 3)],
        "RingCommutative" => vec![("prop_mul_is_commutative", 2)],
        _ => vec![],
    }.into_iter()
        .map(|(n, p)| {
            (
                Ident::new(format!("Abstract{}", tra1t)),
                Ident::new(format!("{}_approx", n)),
                p,
            )
        })
        .collect()
}

/// Implementation of the custom derive
#[proc_macro_derive(Alga, attributes(alga_traits, alga_quickcheck))]
pub fn derive_alga(input: TokenStream) -> TokenStream {
    use syn::MetaItem::*;
    use syn::NestedMetaItem::*;
    use syn::Lit::*;

    let item = syn::parse_derive_input(&input.to_string()).unwrap();
    let name = &item.ident;
    let (i, t, w) = item.generics.split_for_impl();
    let (impl_generics, ty_generics) = (once(&i).cycle(), once(&t).cycle());

    let iter = item.attrs
        .iter()
        .filter_map(|a| {
            if let List(ref ident, ref traits) = a.value {
                Some((ident, traits))
            } else {
                None
            }
        })
        .filter(|&(i, _)| i == "alga_traits")
        .flat_map(|(_, v)| v)
        .map(|t| match *t {
            MetaItem(ref m) => match *m {
                List(ref name, ref value) => (name.as_ref(), value.clone()),
                NameValue(ref name, ref value) => {
                    if name == "Where" {
                        (name.as_ref(), vec![Literal(value.clone())])
                    } else {
                        panic!("Where clause should be defined with `Where = \"TypeParameter: Trait\"`.");
                    }
                }
                Word(ref i) => {
                    let oper = match get_op_arity(&format!("{}", i)) {
                        1 => "Operator",
                        2 => "Operator1, Operator2",
                        n => unreachable!("Trait `{}` with unknown arity {} encountered.", name, n),
                    };
                    panic!(
                        "Operator has to be provided via #[alga_traits({}({}))]",
                        i, oper
                    );
                }
            },
            _ => {
                panic!("Derived alga trait has to be provided via #[alga_traits(Trait(Operators))]")
            }
        });
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
                } else {
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
            let value: Vec<_> = value
                .iter()
                .map(|v| {
                    if let MetaItem(Word(ref ident)) = *v {
                        ident.clone()
                    } else {
                        panic!(
                            "Operator has to be provided via #[alga_traits({}({}))].",
                            name,
                            value
                                .iter()
                                .map(|v| match *v {
                                    MetaItem(ref i) => i.name(),
                                    Literal(Str(ref i, _)) => i,
                                    _ => "Operator",
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                })
                .collect();
            traits.push((name, value, None));
        }
    }
    let (tra1t, op, where_clause, checks) = traits
        .into_iter()
        .flat_map(|(name, value, clause)| {
            let arity = get_op_arity(name);
            let value = value.clone();
            if value.len() != arity {
                match arity {
                    1 => {
                        let message = format!("One operator is required for `{}` trait.", name);
                        match value.len() {
                            0 => panic!("{} None was provided.", message),
                            _ => panic!("{} Too many were provided.", message),
                        }
                    }
                    2 => {
                        let message = format!("Two operators are required for `{}` trait.", name);
                        match value.len() {
                            0 => panic!("{} None was provided.", message),
                            1 => panic!("{} Only one was provided.", message),
                            _ => panic!("{} Too many were provided.", message),
                        }
                    }
                    n => unreachable!("Trait `{}` with unknown arity {} encountered.", name, n),
                }
            }
            let create_tuple = |n: &str, i: usize| {
                let mul = if i == 1 { value.first().cloned() } else { None };
                let value = if get_op_arity(n) == 1 {
                    vec![value[i].clone()]
                } else {
                    value.clone()
                };
                (
                    Ident::from(format!("Abstract{}", n)),
                    value.clone(),
                    clause.clone(),
                    (value, mul, get_props(n)),
                )
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
                        .map(|n| create_tuple(&n, 1)),
                ).collect()
            }
        })
        .unzip4();
    assert!(!tra1t.is_empty(),
    "Atleast one trait is required to be implemented.\n         Trait can be specified with `#[alga_traits(Trait(Operators))]` attribute.");

    let dummy_const = Ident::new(format!("_ALGA_DERIVE_{}", name));
    let type_name = once(&name).cycle();
    let mut tks = quote!(
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate alga as _alga;
            #(
                #[automatically_derived]
                impl #impl_generics _alga::general::#tra1t<#(#op,)*> for #type_name #ty_generics #where_clause {}
            )*
        };
    );

    if let Some((_, checked_generics)) = item.attrs
        .iter()
        .filter_map(|a| match a.value {
            Word(ref name) => Some((name, None)),
            List(ref name, ref value) => Some((name, Some(value))),
            _ => None,
        })
        .filter(|&(n, _)| n.as_ref() == "alga_quickcheck")
        .next()
    {
        let checked_generics = checked_generics
            .map(|checks| {
                let err = "To specify which concrete types are used for generic parameters `#[alga_quickcheck(check(Type1, Type2))]` form should be used.";
                checks
                    .iter()
                    .map(|ty_params| {
                        if let MetaItem(List(ref indicator, ref tys)) = *ty_params {
                            if indicator == "check" {
                                tys.iter()
                                    .map(|ty| {
                                        if let MetaItem(Word(ref ident)) = *ty {
                                            ident.clone()
                                        } else {
                                            panic!("Concrete types has to be provided via #[alga_quickcheck(check({}))].", tys.iter().map(|v| match *v {
                                MetaItem(ref i) => i.name(),
                                Literal(Str(ref i, _)) => i,
                                _ => "Type",
                            }).collect::<Vec<_>>().join(", "));
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            } else {
                                panic!(err);
                            }
                        } else {
                            panic!(err);
                        }
                    })
                    .collect()
            })
            .unwrap_or(vec![]);
        for (ops, add, check) in checks {
            let ops = &ops;
            for (tra1t, check, nparams) in check {
                let mut add_test = |check_generics: &[Ident]| {
                    let params: &Vec<_> = &(0..nparams).map(|_| name).collect();
                    let nparams: &Vec<_> = &(0..nparams)
                        .map(|n| Ident::new(format!("v{}", n)))
                        .collect();
                    let show_ops: String = ops.iter().map(|n| format!("_{}", n)).collect();
                    let mut name_gens = check_generics
                        .iter()
                        .map(|g| g.to_string())
                        .collect::<Vec<_>>()
                        .join("_");
                    if !name_gens.is_empty() {
                        name_gens = format!("_{}", name_gens);
                    }
                    let test_name = Ident::new(format!(
                        "{}_for_{}{}_as_{}{}",
                        check, name, name_gens, tra1t, show_ops
                    ));
                    let check_generics = Generics {
                        ty_params: check_generics.iter().cloned().map(Into::into).collect(),
                        ..Default::default()
                    };
                    let generics1 = once(&check_generics).cycle();
                    let generics2 = once(&check_generics).cycle();
                    let nonzero = if let Some(ref add) = add {
                        let add = once(add).cycle();
                        quote!(
                            {
                                let &(#(ref #nparams,)*) = &args;
                                #(
                                    if #nparams == &_alga::general::Identity::<#add>::identity() {
                                        return _quickcheck::TestResult::discard();
                                    }
                                )*
                            }
                        )
                    } else {
                        quote!()
                    };
                    let parsed: String = quote!(
                        #[test]
                        #[allow(non_snake_case)]
                        fn #test_name() {
                            extern crate quickcheck as _quickcheck;
                            extern crate alga as _alga;
                            fn prop(args: (#(#params #generics1,)*)) -> _quickcheck::TestResult {
                                #nonzero
                                _quickcheck::TestResult::from_bool(_alga::general::#tra1t::<#(#ops),*>::#check(args))
                            }
                            _quickcheck::quickcheck(prop as fn((#(#params #generics2,)*)) -> _quickcheck::TestResult);
                        }
                    ).parse()
                        .unwrap();
                    tks.append(&parsed);
                };
                if checked_generics.is_empty() {
                    add_test(&vec![][..]);
                } else {
                    for check_generics in &checked_generics {
                        add_test(check_generics);
                    }
                }
            }
        }
    }
    tks.parse().unwrap()
}

trait Unzip4<A, B, C, D> {
    fn unzip4(self) -> (Vec<A>, Vec<B>, Vec<C>, Vec<D>);
}

impl<A, B, C, D, I> Unzip4<A, B, C, D> for I
where
    I: Iterator<Item = (A, B, C, D)>,
{
    fn unzip4(self) -> (Vec<A>, Vec<B>, Vec<C>, Vec<D>) {
        let hint = self.size_hint().1.unwrap_or(Vec::<A>::new().capacity());
        let (mut va, mut vb, mut vc, mut vd) = (
            Vec::with_capacity(hint),
            Vec::with_capacity(hint),
            Vec::with_capacity(hint),
            Vec::with_capacity(hint),
        );
        for (a, b, c, d) in self {
            va.push(a);
            vb.push(b);
            vc.push(c);
            vd.push(d);
        }
        (va, vb, vc, vd)
    }
}
