# Mediator Pattern

## Rust Idiomatic Approach

The key point is thinking in terms of OWNERSHIP.

1. A mediator takes ownership of all components.
2. A component doesn't preserve a reference to a mediator. Instead, it gets the reference via a method call.
3. Control flow starts from the `fn main()` where the mediator receives external events/commands.
4. Mediator trait for the interaction between components is not the same as its external API for receiving external events (commands from the main loop).

See the full article: [README.md](../README.md).

👉 A real-world example of such approach: [Cursive (TUI)][5].

![Ownership model in Rust](../images/mediator-rust-approach.jpg)

[5]: https://crates.io/crates/cursive
