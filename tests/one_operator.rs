

macro_rules! check_int {
    ($($T:ident),* $(,)*) => {
        $(mod $T {
            use ops::Additive;
            use general::AbstractQuasigroup;

            #[quickcheck]
            fn prop_inv_is_latin_square(args: ($T, $T)) -> bool {
                AbstractQuasigroup::<Additive>::prop_inv_is_latin_square(args)
            }
        })+
    }
}

//check_int!(u8);
//check_int!(u16);
//check_int!(u32);
//check_int!(u64);
check_int!(i8, i16, i32, i64);

macro_rules! check_int {
    ($($T:ident),* $(,)*) => {
        $(
            #[cfg(test)]
            mod $T {
                use ops::{Additive, Multiplicative};
                use general::AbstractMonoid;

                #[quickcheck]
                fn prop_zero_is_noop(args: $T) -> bool {
                    AbstractMonoid::<Additive>::prop_operating_identity_element_is_noop(args)
                }

                #[quickcheck]
                fn prop_mul_unit_is_noop(args: $T) -> bool {
                    AbstractMonoid::<Multiplicative>::prop_operating_identity_element_is_noop(args)
                }
            }
        )+
    }
}

check_int!(u8, u16, u32, u64, i8, i16, i32, i64);

macro_rules! check_int {
    ($($T:ident),* $(,)*) => {
        $(mod $T {
            use ops::{Additive, Multiplicative};
            use general::AbstractSemigroup;

            #[quickcheck]
            fn prop_add_is_associative(args: ($T, $T, $T)) -> bool {
                AbstractSemigroup::<Additive>::prop_is_associative(args)
            }

            #[quickcheck]
            fn prop_mul_is_associative(args: ($T, $T, $T)) -> bool {
                AbstractSemigroup::<Multiplicative>::prop_is_associative(args)
            }
        })+
    }
}
check_int!(u8, u16, u32, u64, i8, i16, i32, i64);
