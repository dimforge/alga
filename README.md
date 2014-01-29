# num-rs

[![Build Status](https://travis-ci.org/bjz/num-rs.png?branch=master)](https://travis-ci.org/bjz/num-rs)

After the traits in `std::num` have been simplified, this library is intended to 
provide a strong algebraic foundation for numeric libraries. Hopefully this will
be then be bundled with Rust in a separate module like `extra::num` or
`extra::algebra`.

Whilst many of the trait names names used in the library are intimidating, the
most programmers will only need to use the traits defined in the `num` module.

## Overview

Abstract algebra organises organises a wide range of structures into
a logically consistent framework. These classifications can be incredibly
creating composable libraries and APIs.

Examples of types that could be included under the algebraic framework:

- Integers, reals and rationals numbers
- Complex numbers
- Polynomials
- Boolean values
- Matrices and vectors
- Quaternions and octonians
- Strings under concatenation

## Open questions

- How do we handle the imperfectness of floating point numbers, and types
  parameterised over them, for example `Vector3<f32>`?
- How do we handle integer overflow?

## Research

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
