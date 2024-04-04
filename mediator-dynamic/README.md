# Mediator

## Mimicking a Typical OOP

It implements a [Station Manager example in Go][4].

🏁 I would recommend this approach for applications that need **multi-threaded support**: particular components after being added to the mediator can be sent to different threads and modified from there.

📄 Real-world example: [`indicatif::MultiProgress`](https://docs.rs/indicatif/latest/indicatif/struct.MultiProgress.html) (mediates progress bars with support of being used in multiple threads).

Key points:

1. All trait methods look like read-only (`&self`): immutable `self` and parameters.
2. `Rc`, `RefCell` are extensively used under the hood to take responsibility for the mutable borrowing from compilation time to runtime. Invalid implementation will lead to panic in runtime.

See the full article: [README.md](../README.md).

[4]: https://refactoring.guru/design-patterns/mediator/go/example
