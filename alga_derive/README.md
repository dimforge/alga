<p align="center">
    <a href="https://crates.io/crates/alga">
         <img src="http://meritbadge.herokuapp.com/alga?style=flat-square" alt="crates.io">
    </a>
    <a href="https://travis-ci.org/sebcrozet/alga">
        <img src="https://travis-ci.org/sebcrozet/alga.svg?branch=master" alt="Build status">
    </a>
</p>
<p align = "center">
    <strong>
        <a href="https://docs.rs/alga">Documentation</a>
    </strong>
</p>

alga − abstract algebra for Rust
========

**alga** aim to provide solid mathematical abstractions to algebra-focused
applications. It defines and organize through trait inheritance the basic
building blocks of general algebraic structures. Specific implementation of
algebraic structure traits is is left to other crates. Higher-level traits for
specilazied domain of algebra (like linear algebra) are also provided and will
prove useful for applications that include code that is generic wrt. the
algebraic entity types.

Examples of types that could derive from traits defined in **alga**:

- Integers, reals and rationals numbers
- Complex numbers
- Polynomials
- Matrices and vectors
- Quaternions and octonians
- Strings under concatenation

**alga** started as a fork of [algebra](https://crates.io/crates/algebra).
