# Mediator Patter in Rust

_*Mediator* is a challenging pattern to be implemented in *Rust*._ Here is **why** and **what the ways** are there to implement Mediator in Rust.

By definition, [Mediator][1] restricts direct communications between the objects and forces them to collaborate only via a mediator object. It also stands for a Controller in the MVC pattern. Let's see the nice diagrams from https://refactoring.guru:

| Problem | Solution |
|---------|----------|
|![image](https://user-images.githubusercontent.com/5967447/182462245-cec68da8-73a9-4abf-9c18-dd5303d919fb.png)|![image](https://user-images.githubusercontent.com/5967447/182462302-c7ba0c25-095e-47af-ac8a-9a560a987036.png)|

## Problem

A common implementation in object-oriented languages looks like the following (pseudocode!):

```java
Controller controller = new Controller();

// Every compenent has a link to a mediator (controller).
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

![image](https://user-images.githubusercontent.com/5967447/182466067-fa4f7b25-58bb-4272-be8d-b0a19c0c50f0.png)

1. Imagine that the control flow starts at the point 1 (Checkbox) where the 1st **mutable** borrow happens.
2. The mediator (Dialog) interacts with another object at the point 2 (TextField).
3. The TextField notifies the Dialog back about finishing job and that leads to a **mutable** action at the point 3... Bang!

The second mutable borrow breaks the compilation with an error (the first borrow was on the point 1).

**In Rust, a widespread Mediator implementation is mostly an anti-pattern.**

## Existing Primers

You might see a reference Mediator examples in Rust like [this][5]: the example is too much synthetic - there is not mutable operations, at least at the level of the trait methods.

The [rust-unofficial/patterns](https://github.com/rust-unofficial/patterns) repository doesn't include a referenced Mediator pattern implementation as of now, see the [Issue #233][2].

**Nevertheless, we don't surrender.**

## Option 1 (1st Try): Direct implementation "like in OOP"

There is an example of a [Railway Station manager example][4] in Go language.

👉 I managed to implement it in Rust: [mediator-like-in-go](https://github.com/fadeevab/mediator-pattern-rust/mediator-like-in-go)

⚠ I wouldn't recommend this approach, however, I think it's a good reference of how the Rust compiler could be tricked.

Key points:

1. All methods are read-only: immutable `self` and parameters.
2. `Rc`, `RefCell` are extensively used under the hood to take responsibility for the mutable borrowing from compiler to runtime. Invalid implementation will lead to panic in runtime.

## Option 2: Rust Idiomatic Approach

⚠ The key point is thinking about OWNERSHIP.

1. Mediator takes ownership over all other objects (components).
2. Components don't store a reference to a mediator, instead, they get the reference via a callback.
3. Control flow start from the Mediator.

![Ownership](https://user-images.githubusercontent.com/5967447/182478197-ae4f5969-a46e-4c32-9948-b356793a05ed.jpg)

A few changes of the direct approach lead to a safe mutability checked in compilation time.

👉 Railway Station example reworked **without** `Rc`, `RefCell` tricks, but **with** `&mut self` and compiler-time borrow checking: https://github.com/fadeevab/mediator-pattern-rust/mediator.

[1]: https://refactoring.guru/design-patterns/mediator
[2]: https://github.com/rust-unofficial/patterns/issues/233
[3]: https://chercher.tech/rust/mediator-design-pattern-rust
[4]: https://refactoring.guru/design-patterns/mediator/go/example
[5]: https://crates.io/crates/cursive
