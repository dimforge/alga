use general::{AbstractMagma, AbstractQuasigroup, AbstractLoop, AbstractSemigroup,
              AbstractMonoid, AbstractGroup, AbstractGroupAbelian, Multiplicative, Additive,
              AbstractRing, AbstractRingCommutative, AbstractField, AbstractModule,
              ClosedAdd, ClosedSub, ClosedNeg, ClosedMul, ClosedDiv};

macro_rules! specialize_structures(
    // **With type parameters** for the trait being implemented.
    ($specialized: ident, $abstract_trait: ident<$($ops: ident),*> : $($bounds: ident)*) => {
        /// [Alias] Algebraic structure specialized for one kind of operation.
        pub trait $specialized: $abstract_trait<$($ops),*> $(+ $bounds)* { }
        impl<T: $abstract_trait<$($ops),*> $(+ $bounds)*> $specialized for T { }
    };
    // **Without type parameters** for the trait being implemented.
    ($specialized: ident, $abstract_trait: ident : $($bounds: ident)*) => {
        /// [Alias] Algebraic structure specialized for one kind of operation.
        pub trait $specialized: $abstract_trait $(+ $bounds)* { }
        impl<T: $abstract_trait $(+ $bounds)*> $specialized for T { }
    }
);


specialize_structures!(AdditiveMagma,        AbstractMagma<Additive>        : );
specialize_structures!(AdditiveQuasigroup,   AbstractQuasigroup<Additive>   : AdditiveMagma ClosedSub);
specialize_structures!(AdditiveLoop,         AbstractLoop<Additive>         : AdditiveQuasigroup ClosedNeg);
specialize_structures!(AdditiveSemigroup,    AbstractSemigroup<Additive>    : AdditiveMagma ClosedAdd);
specialize_structures!(AdditiveMonoid,       AbstractMonoid<Additive>       : AdditiveSemigroup);
specialize_structures!(AdditiveGroup,        AbstractGroup<Additive>        : AdditiveLoop AdditiveMonoid);
specialize_structures!(AdditiveGroupAbelian, AbstractGroupAbelian<Additive> : AdditiveGroup);


specialize_structures!(MultiplicativeMagma,      AbstractMagma<Multiplicative>      : );
specialize_structures!(MultiplicativeQuasigroup, AbstractQuasigroup<Multiplicative> : MultiplicativeMagma ClosedDiv);
specialize_structures!(MultiplicativeLoop,       AbstractLoop<Multiplicative>       : MultiplicativeQuasigroup);
specialize_structures!(MultiplicativeSemigroup,  AbstractSemigroup<Multiplicative>  : MultiplicativeMagma ClosedMul);
specialize_structures!(MultiplicativeMonoid,     AbstractMonoid<Multiplicative>     : MultiplicativeSemigroup);
specialize_structures!(MultiplicativeGroup,      AbstractGroup<Multiplicative>      : MultiplicativeLoop MultiplicativeMonoid);
specialize_structures!(MultiplicativeGroupAbelian, AbstractGroupAbelian<Multiplicative> : MultiplicativeGroup);


specialize_structures!(Ring,            AbstractRing:            AdditiveGroupAbelian MultiplicativeMonoid);
specialize_structures!(RingCommutative, AbstractRingCommutative: Ring);
specialize_structures!(Field,           AbstractField:           RingCommutative MultiplicativeGroupAbelian);

/// A module which overloads the `*` and `+` operators.
pub trait Module: AbstractModule<AbstractRing = <Self as Module>::Ring> +
                  ClosedMul<<Self as Module>::Ring> {
    /// The underlying scalar field.
    type Ring: RingCommutative;
}
// FIXME: unfortunately, Module cannot be auto-impl-ed.
