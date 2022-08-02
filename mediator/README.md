# Mediator Pattern

Rust Idiomatic Approach

⚠ The key point is thinking about OWNERSHIP.

1. Mediator takes ownership over all other objects (components).
2. Components don't store a reference to a mediator, instead, they get the reference via a callback.
3. Control flow start from the Mediator.

See the full article: [README.md](../README.md).