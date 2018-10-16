use general::{AbstractGroupAbelian, AbstractRingCommutative, Additive, Multiplicative, Operator};

/// A module combines two sets: one with an Abelian group structure and another with a
/// commutative ring structure.
///
/// `OpGroup` denotes the Abelian group operator (usually the addition). In addition, and external
/// multiplicative law noted `∘` is defined. Let `S` be the ring with multiplicative operator
/// `OpMul` noted `×`, multiplicative identity element noted `1`, and additive operator `OpAdd`.
/// Then:
///
/// ```notrust
/// ∀ a, b ∈ S
/// ∀ x, y ∈ Self
///
/// a ∘ (x + y) = (a ∘ x) + (a ∘ y)
/// (a + b) ∘ x = (a ∘ x) + (b ∘ x)
/// (a × b) ∘ x = a ∘ (b ∘ x)
/// 1 ∘ x       = x
/// ```
pub trait AbstractModule<
    OpGroup: Operator = Additive,
    OpAdd: Operator = Additive,
    OpMul: Operator = Multiplicative,
>: AbstractGroupAbelian<OpGroup> {
    /// The underlying scalar field.
    type AbstractRing: AbstractRingCommutative<OpAdd, OpMul>;

    /// Multiplies an element of the ring with an element of the module.
    fn multiply_by(&self, r: Self::AbstractRing) -> Self;
}
