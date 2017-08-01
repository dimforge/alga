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
    .map(|(n, p)| (Ident::new(format!("Abstract{}", tra1t)), Ident::new(format!("{}_approx", n)), p))
    .collect()
}

#[proc_macro_derive(Alga, attributes(alga_traits, alga_quickcheck))]
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
            match *t {
                MetaItem(ref m) => match *m {
                    List(ref name, ref value) => (name.as_ref(), value.clone()),
                    NameValue(ref name, ref value) => if name == "Where" {
                        (name.as_ref(), vec![Literal(value.clone())])
                    } else {
                        panic!("Where clause should be defined with `Where = \"TypeParameter: Trait\"`.");
                    },
                    Word(ref i) => {
                        let oper = match get_op_arity(&format!("{}", i)) {
                            1 => "Operator",
                            2 => "Operator1, Operator2",
                            n => unreachable!("Trait `{}` with unknown arity {} encountered.", name, n),
                        };
                        panic!("Operator has to be provided via #[alga_traits({}({}))]", i, oper);
                    },
                },
                _ => panic!("Derived alga trait has to be provided via #[alga_traits(Trait(Operators))]"),
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
            traits.push((name, value, None));
        }
    }
    let (tra1t, op, where_clause, checks) = traits.into_iter()
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
                    },
                    2 => {
                        let message = format!("Two operators are required for `{}` trait.", name);
                        match value.len() {
                            0 => panic!("{} None was provided.", message),
                            1 => panic!("{} Only one was provided.", message),
                            _ => panic!("{} Too many were provided.", message),
                        }
                    },
                    n => unreachable!("Trait `{}` with unknown arity {} encountered.", name, n),
                }
            }
            let create_tuple = |n: &str, i: usize| {
                let mul = if i == 1 {
                    value.first().cloned()
                } else {
                    None
                };
                let value = if get_op_arity(n) == 1 {
                    vec![value[i].clone()]
                } else {
                    value.clone()
                };
                (Ident::from(format!("Abstract{}", n)), value.clone(), clause.clone(), (value, mul, get_props(n)))
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

    if let Some(_) = item.attrs.iter()
            .filter_map(|a|
                if let Word(ref ident) = a.value {
                    Some(ident)
                } else {
                    None
                })
            .filter(|i| i.as_ref() == "alga_quickcheck")
            .next() {
        for (ops, add, check) in checks {
            let ops = &ops;
            for (tra1t, check, nparams) in check {
                let params: &Vec<_> = &(0..nparams).map(|_| name).collect();
                let nparams: &Vec<_> = &(0..nparams).map(|n| Ident::new(format!("v{}", n))).collect();
                let show_ops: String = ops.iter()
                    .map(|n| match n {
                        &MetaItem(Word(ref ident)) => format!("_{}", ident),
                        _ => panic!(),
                    })
                    .collect();
                let dummy_const = Ident::new(format!("{}_for_{}_as_{}{}", check, name, tra1t, show_ops));
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
                    fn #dummy_const() {
                        extern crate quickcheck as _quickcheck;
                        extern crate alga as _alga;
                        fn prop(args: (#(#params,)*)) -> _quickcheck::TestResult {
                            #nonzero
                            _quickcheck::TestResult::from_bool(_alga::general::#tra1t::<#(#ops),*>::#check(args))
                        }
                        _quickcheck::quickcheck(prop as fn((#(#params,)*)) -> _quickcheck::TestResult);
                    }
                ).parse().unwrap();
                tks.append(&parsed);
            }
        }
    }
    tks.parse().unwrap()
}

trait Unzip4<A, B, C, D> {
    fn unzip4(self) -> (Vec<A>, Vec<B>, Vec<C>, Vec<D>);
}

impl<A, B, C, D, I> Unzip4<A, B, C, D> for I
    where I: Iterator<Item=(A, B, C, D)>,
{
    fn unzip4(self) -> (Vec<A>, Vec<B>, Vec<C>, Vec<D>) {
        let hint = self.size_hint().1.unwrap_or(Vec::<A>::new().capacity());
        let (mut va, mut vb, mut vc, mut vd) = (
            Vec::with_capacity(hint),
            Vec::with_capacity(hint),
            Vec::with_capacity(hint),
            Vec::with_capacity(hint)
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
