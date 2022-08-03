# Mediator Pattern in Rust

_*Mediator* is a challenging pattern to be implemented in *Rust*._ Here is **why** and **what ways** are there to implement Mediator in Rust.

A typical Mediator implementation in other languages is a classic anti-pattern in Rust: many objects hold mutable cross-references on each other, trying to mutate each other, which is a deadly sin in Rust - the compiler won't pass your first naive implementation unless it's oversimplified.

By definition, [Mediator][1] restricts direct communications between the objects and forces them to collaborate only via a mediator object. It also stands for a Controller in the MVC pattern. Let's see the nice diagrams from https://refactoring.guru:

| Problem                      | Solution                      |
| ---------------------------- | ----------------------------- |
| ![image](images/problem.png) | ![image](images/solution.png) |

## Problem

A common implementation in object-oriented languages looks like the following (pseudo-code!):

```java
Controller controller = new Controller();

// Every component has a link to a mediator (controller).
component1.setController(controller);
component2.setController(controller);
component3.setController(controller);

// A mediator has a link to every object.
controller.add(component1);
controller.add(component2);
controller.add(component2);
```

Now, let's read this in **Rust** terms: _"**mutable** structures have **mutable** references to a **shared mutable** object (mediator) which in turn has mutable references back to those mutable structures"_.

Basically, you can start to imagine the unfair battle against the Rust compiler and its borrow checker. It seems like a solution introduces more problems:

![image](images/mediator-mut-problem.png)

1. Imagine that the control flow starts at point 1 (Checkbox) where the 1st **mutable** borrow happens.
2. The mediator (Dialog) interacts with another object at point 2 (TextField).
3. The TextField notifies the Dialog back about finishing a job and that leads to a **mutable** action at point 3... Bang!

The second mutable borrow breaks the compilation with an error (the first borrow was on the point 1).

**In Rust, a widespread Mediator implementation is mostly an anti-pattern.**

## Existing Primers

You might see a reference Mediator examples in Rust like [this][5]: the example is too much synthetic - there are no mutable operations, at least at the level of trait methods.

The [rust-unofficial/patterns](https://github.com/rust-unofficial/patterns) repository doesn't include a referenced Mediator pattern implementation as of now, see the [Issue #233][2].

**Nevertheless, we don't surrender.**

## Option 1 (1st Try): `Rc<RefCell<..>>` Hidden Under Read-Only Methods

There is an example of a [Station Manager example in Go][4]. Trying to convert it into Rust with a direct approach leads to mimicking a typical OOP through reference counting and borrow checking with mutability in runtime (which has quite unpredictable behavior in runtime with panics here and there).

👉 Here is a Rust implementation: [mediator-like-in-go](https://github.com/fadeevab/mediator-pattern-rust/mediator-like-in-go)

⚠ I wouldn't recommend this approach, however, I think it's a good reference of how the Rust compiler could be tricked.

Key points:

1. All trait methods are **read-only**: immutable `self` and immutable parameters.
2. `Rc`, `RefCell` are extensively used under the hood to take responsibility for the mutable borrowing from compiler to runtime. Invalid implementation will lead to panic in runtime.

## Option 2: Top-Down Ownership

☝ The key point is thinking in terms of OWNERSHIP.

![Ownership](images/mediator-rust-approach.jpg)

1. A mediator takes ownership of all components.
2. A component doesn't preserve a reference to a mediator. Instead, it gets the reference via a method call.
3. Control flow starts from the `fn main()` where the mediator receives external events/commands.
4. Mediator trait for the interaction between components is not the same as its external API for receiving external events (commands from the main loop).

A few changes to the direct approach leads to a safe mutability being checked at compilation time.

👉 A Train Station primer **without** `Rc`, `RefCell` tricks, but **with** `&mut self` and compiler-time borrow checking: https://github.com/fadeevab/mediator-pattern-rust/mediator-recommended.

👉 A real-world example of such approach: [Cursive (TUI)][5].

[1]: https://refactoring.guru/design-patterns/mediator
[2]: https://github.com/rust-unofficial/patterns/issues/233
[3]: https://chercher.tech/rust/mediator-design-pattern-rust
[4]: https://refactoring.guru/design-patterns/mediator/go/example
[5]: https://crates.io/crates/cursive
