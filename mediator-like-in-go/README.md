# Mediator like in Go example

A direct (naive?) implementation with Rust of a [Railway Station manager example in Go][4].

⚠ I wouldn't recommend this approach, however, I think it's a good reference of how the Rust compiler could be tricked.

Key points:

1. All methods are read-only: immutable `self` and parameters.
2. `Rc`, `RefCell` are extensively used under the hood to take responsibility for the mutable borrowing from compiler to runtime. Invalid implementation will lead to panic in runtime.

See the full article: [README.md](../README.md).

[4]: https://refactoring.guru/design-patterns/mediator/go/example