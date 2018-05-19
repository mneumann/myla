# My Language (MyLa)

My experiments with programming language design.

## Inspirations

* Languages that lack exceptions (Rust, Go, Zig). TODO: Why are
  exceptions evil?

* Type-driven development (Idris).

* Dependent types (Idris).

* Readability (Ada).

* Concept programming (XL): "Code should reflect the concepts of your
  domain". [Good presentation][1].

* Design by Contract (Eiffel).

* Pony: No operator precedence, no shadowing of variables.

* Concatenative languages (Kitten).

* Principle of Least Surprise (Ruby).

* Uniqueness types (Concurrent Clean).

## Requirements

* Reasonable fast (max. 2x slower than native).

* Ability to compile down to C, to make bootstrapping easy.

* Early-on written in itself ("Eat your own dog food").

* Not tied to any particular platform (.NET, JVM).

## Random ideas

* Incorrectly formatted source code is rejected as compile error. This
  leads to uniformly formatted source code, which is mandantory. A tool
  like ```rustfmt``` can help with automatic formatting.

* The compiler is primarily a library, not a CLI.

[1]: http://xlr.sourceforge.net/Concept%20Programming%20Presentation.pdf
