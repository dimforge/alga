<p align="center">
    <a href="https://crates.io/crates/alga-derive">
         <img src="http://meritbadge.herokuapp.com/alga-derive?style=flat-square" alt="crates.io">
    </a>
    <a href="https://travis-ci.org/sebcrozet/alga">
        <img src="https://travis-ci.org/sebcrozet/alga.svg?branch=master" alt="Build status">
    </a>
</p>
<p align = "center">
    <strong>
        <a href="https://docs.rs/alga-derive">Documentation</a>
    </strong>
</p>

alga-derive − automatic deriving of abstract algebra traits for Rust
========

**alga-derive** allows automatic deriving of traits provided by **alga**.

It supports deriving following **alga** traits:

- `AbstractQuasigroup`
- `AbstractMonoid`
- `AbstractSemigroup`
- `AbstractGroup`
- `AbstractGroupAbelian`
- `AbstractRing`
- `AbstractRingCommutative`
- `AbstractField`

The custom derive can also be used to generate **quickcheck** tests that check
that algebraic properties are satisfied by the target of the derive.
