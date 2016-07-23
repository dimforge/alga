macro_rules! impl_marker {
    ($M:ty; $($T:ty,)+) => {
        $(impl $M for $T {})+
    }
}

macro_rules! impl_ident {
    ($M:ty; $V:expr; $($T:ty,)+) => {
        $(impl Identity<$M> for $T { #[inline] fn id() -> $T {$V} })+
    }
}
