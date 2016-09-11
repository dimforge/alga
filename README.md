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

## References

### Interesting papers

- [The Scratchpad II Type System: Domains and Subdomains](http://www.csd.uwo.ca/~watt/pub/reprints/1990-miola-spadtypes.pdf)
- [Fundamental Algebraic Concepts in Concept-Enabled C++](ftp://cgi.cs.indiana.edu/pub/techreports/TR638.pdf)

### Inspiring Libraries

- [Numeric Prelude](http://www.haskell.org/haskellwiki/Numeric_Prelude) (Haskell)
- Edward A. Kmett's [algebra package](http://hackage.haskell.org/package/algebra-3.1) (Haskell)
- [YAP: Yet Another Prelude](http://hackage.haskell.org/package/yap) (Haskell)
- Agda's [algebra module](http://www.cse.chalmers.se/~nad/listings/lib-0.7/Algebra.html) (Agda)
- Idris' [algebra module](https://github.com/idris-lang/Idris-dev/blob/master/libs/prelude/Prelude/Algebra.idr) (Idris)
- Felix's [algebra module](http://felix-lang.org/$/usr/local/lib/felix/felix-latest/share/lib/std/algebraic.flx) (Felix)
- [non/spire](https://github.com/non/spire) (Scala)
