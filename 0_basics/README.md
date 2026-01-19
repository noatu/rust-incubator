Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

- [x] What memory model does Rust have? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
    - Rust uses the same memory model as C or C++ do (based on LLVM). Memory is organized as stack, heap, and static storage. The ownership system enforces memory safety at compile time.
    - The Rust standard library additionally links to the C standard library. Rust code can be compiled **without** the standard library, in which case the runtime is roughly equivalent to C's.
    - Rust supports both single-threaded and multiple-threaded execution. Thread safety is enforced with `Send` and `Sync` auto marker traits, which prevent data races at compile time.
    - Rust is synchronous by default. Async support is opt-in. `async` and `await` may be used to write functions, but executing them requires an async runtime, such as Tokio. Under the hood async functions are essentially state machines. They return Futures that must be polled to advance the state machine to completion.

- [x] What runtime does Rust have? Does it use a GC (garbage collector)?
    - Sources:
        - [FAQ Is Rust garbage collected?](https://prev.rust-lang.org/en-US/faq.html#is-rust-garbage-collected)
        - [FAQ Does Rust have a runtime?](https://prev.rust-lang.org/en-US/faq.html#does-rust-have-a-runtime)
    - Rust has no runtime system in the typical sense used by languages such as Java, but parts of the Rust standard library can be considered a "runtime", providing a heap, backtraces, unwinding, and stack guards. It does not have a GC, and does not need any to be memory safe (as in no segfaults). Rust uses a system of ownership and borrowing instead, and that system solved many other problems such as resource management and concurrency. That makes Rust lean and easy to embed and integrate.
    - There is, however a possibility of having an optional garbage collection as an extension, to enable even better integration with garbage-collected runtimes (e.g., Spidermonkey and V8 JS engines).

- [x] What does static typing mean? What is a benefit of using it?
    - Static typing means that variable have fixed, known types that are checked at compile-time.
    - It catches type errors before execution. Given the Rust's powerful type system, entire classes of bugs can be prevented. Static typing also improves performance, since the compiler knows exact memory layouts and can optimize accordingly.

- [x] What are generics and parametric polymorphism? Which problems do they solve?
    - Generics are type parameters that let you write code that works with multiple types.
    - Parametric polymorphism is the formal term for this concept -- functions and types parameterized by other types, i.e. `Vec<T>`.
    - It eliminates code duplication, maintains type safety (the compiler validates the operations based on concrete types), and enables abstraction without performance cost (with monomorphization, which may result in an increased binary size, though).

- [x] What are traits? How are they used? How do they compare to interfaces? What are auto traits and blanket impls? What is a marker trait?
    - Sources: [Brandon Smith: Three Kinds of Polymorphism in Rust](https://www.brandons.me/blog/polymorphism-in-rust), "trust me bro".
    - Traits are the primary abstraction mechanism in Rust. They can be thought of like an interface or protocol from other languages: they specify a set of methods that a struct must implement, and then they can be implemented for arbitrary structs and those structs can be used where the trait is expected.
    - One disadvantage, which will not be obvious coming from other languages: there's no way with a trait to find out which variant you're working with and get at its other properties. There's no `instanceof`, there's no `as` casting. You can only work with the value via the actual trait methods.
    - Both traits and interfaces define contracts types must fulfill. Traits are more powerful though. They support default method implementations, associated types, and can be implemented for external types retroactively.
    - Auto traits are those that are automatically implemented by the compiler. The rule of thumb is if all fields of a struct satisfy an auto trait, then the struct does too. Common ones are `Send`, `Sync`, and `Unpin`.
    - Marker traits have no methods. They are used purely for compile-time guarantees and bounds. Common ones are `Send`, `Sync`, `Copy`, and `Sized`. They signal properties to the type system without adding behaviour.
    - Blanket implementations implement a trait for all types that satisfy the trait bounds. They are not as powerful as one would wish because most of them will conflict with standard library blankets.

- [x] What are static and dynamic dispatch? Which should you use, and when?
    - Source: [Brandon Smith: Three Kinds of Polymorphism in Rust](https://www.brandons.me/blog/polymorphism-in-rust), "trust me bro".
    - Static dispatch resolves which function to call at compile time, the compiler generates separate machine code for each concrete type used with generics. That is fast, enables inlining, has no runtime overhead, but may result in binary bloat.
    - In contrast, dynamic dispatch resolves at runtime using trait objects (`dyn Trait`). It uses a virtual function table (vtable) to look up the correct method. One copy of the code, no binary bloat, yet it has runtime indirection cost which breaks cache-locality and prevents inlining.
    - By default, one should prefer static dispatch, given it's benefits. However static dispatch prevents storing different types together in a heterogeneous collection. The same limitation applies to returning different types within a single function. This can be solved with enums, but enum variants and variant fields are always public, so changing the variants or fields later is not backwards compatible, and others cannot extend your enum. So in those cases, and also when compile-time monomorphization would cause **excessive** code bloat, dynamic dispatch is preferred.
    - It's worth noting, though, that dynamic-dispatch is what most languages always do for this stuff. So even though it's less performant, it's still plenty performant most of the time.

- [x] What is a crate and what is a module in Rust? How do they differ? How are they used?
    - Source: [FAQ What is the relationship between a module and a crate?](https://prev.rust-lang.org/en-US/faq.html#what-is-the-relationship-between-a-module-and-a-crate)
    - A crate is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
    - A module is a (possibly nested) unit of code organization inside a crate.
    - A crate contains an implicit, un-named top-level module (for example we can use `::core`)
    - Recursive definitions can span modules, but not crates.

- [x] What are move semantics? What are borrowing rules? What is the benefit of using them?
    - Sources:
        - [FAQ What is the difference between passing by value, consuming, moving, and transferring ownership?](https://prev.rust-lang.org/en-US/faq.html#what-is-the-difference-between-consuming-and-moving)
        - [FAQ How can I understand the borrow checker?](https://prev.rust-lang.org/en-US/faq.html#how-can-i-understand-the-borrow-checker)
    - Each value must have one owner at a time. When the owner goes out of scope, the value will be dropped.
    - Passing by value (also called consuming, moving, and transferring ownership) means the value has been moved to another owner, and moved out of the possession of the original owner, who can no longer use it. If a type implements the `Copy` trait, the original owner's value won't be invalidated, and can still be used.
    - The borrowing rules are: First, any borrow must last for a scope no greater than that of the owner. Second, you may have one or the other of these two kinds of borrows, but not both at the same time:
        - one or more references (&T) to a resource.
        - exactly one mutable reference (&mut T).
    - Those rules allow data access without taking ownership, enabling multiple references to the same data while preventing data races and ensuring memory safety, which helps avoid common programming errors like dangling pointers and memory leaks.

- [x] What is immutability? What is the benefit of using it?
    - In Rust variables are immutable by default and the `mut` keyword must be used explicitly to allow mutation. That means that, by default, data cannot be changed after creation. This prevents accidental mutations and associated bugs, makes code more reasonable, and enables safe concurrent access.
    - Still, immutable variables can be shadowed for the current scope, but that does not affect the outer scope.

- [x] What is cloning? What is copying? How do they compare?
    - Source: [FAQ Does Rust have copy constructors? ](https://prev.rust-lang.org/en-US/faq.html#does-rust-have-copy-constructors)
    - Types which implement the `Copy` trait will do a standard C-like "shallow copy" with no extra work (similar to trivially copyable types in C++). It is impossible to implement `Copy` types that requires custom copy behavior.
    - In Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, making it easier for the developer to identify potentially expensive operations.
    - Note that cloning a value can be expensive, causing further allocations. The `Copy` trait requires the `Clone` trait to be implemented.

- [x] What is RAII? How is it implemented in Rust? What is the benefit of using it?
    - Source: [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust](https://www.thecodedmessage.com/posts/raii)
    - RAII (Resource Acquisition Is Initialization) is C++'s and now Rust's (somewhat oddly-named) scope-based feature for managing all kinds of resources. Its biggest use case is as part of a compile-time alternative to run-time garbage collection and reference counting.
    - **OBRM** (Ownership-Based Resource Management) is used in Rust sometimes, and is a much better name.
    - In Rust, the move semantics problem (the allocation might not live exactly as long as its owner) is solved via "destructive moves," which means not calling the destructor on the moved-from value. The moved-from value ceases to be a value when it's moved from, and accessing that variable is no longer permitted. The destructor is then called as normal in the place where the value is moved to. This is tracked statically at compile-time in the vast majority of situations, and when it cannot be, an extra boolean is inserted as a "drop flag".
    - All types can be moved and no resources or allocations are ever copied. Instead, moves always have the same implementation: copy the memory that is stored in-line in the value itself, and then do not call the destructor. For copyable types like `int` that do not manage an allocation or other resource, this does amount to a copy, but the original is still not usable. For types with allocations, the pointer or handle is simply brought along bit-by-bit just like other data, and the old value is never touched again, making this a safe operation.
    - While we are borrowing something inside the vector, we can't simultaneously be mutating the vector, which could cause the thing we're borrowing to move. Rust statically ensures that this is impossible, rejecting code that uses a reference after a mutation, destruction, or move somewhere else would invalidate it. This enables us to extend the RAII-based system and both prevent leaks and maintain safety, just like a GC or RC-based system. The borrow checker is essential to doing so.
    - The compiler must be informed about the chain of owners, and about borrowers. At the cost of this annotation, Rust gives you everything a GC does: Allocations are freed when their handles go out of scope, and memory safety is still guaranteed, because the annotations are checked. Memory leaks are as difficult as in a reference counting language, and the annotations are checked, which is most of the benefit of automating them. It's an excellent happy medium between manual memory management and full run-time GC with no run-time cost over a certain discipline of C memory management.

- [x] What is an iterator? What is a collection? How do they differ? How are they used?
    - Sources: [std::collections](https://doc.rust-lang.org/stable/std/collections/index.html), [std::iter](https://doc.rust-lang.org/stable/std/iter/index.html), "trust me bro".
    - A collection is a programming data structure that holds items for data storage and processing. Rust has four major categories: sequences, maps, sets and miscellaneous ([BinaryHeap](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html)). If you have a collection of some kind, and need to perform an operation on the elements of said collection, you will probably use an iterator.
    - Iterators provide a sequence of values in a **generic**, safe, efficient and convenient way. The contents of an iterator are usually lazily evaluated, so that only the values that are actually needed are ever actually produced, and no allocation is needed to temporarily store them.
    - The main difference is that iterators are used to traverse collections without needing to know their internal structure.

- [x] What are macros? Which problems do they solve? What is the difference between declarative and procedural macros?
    - Macros are code that generates other code at compile time. They operate on syntax trees rather than values, allowing metaprogramming in Rust. They enable: reducing repetitive boilerplate, compile-time logic evaluation, variadic functions (accepting any number of arguments), and type introspection.
    - Declarative macros work purely with pattern matching and substitution: they match on syntax with match-like arms and transform input tokens.
    - Procedural macros are rust functions that receive a syntax tree, do manipulations with it, and return a new one. They have three types: function-like, derive, and attribute. Compared to declarative macros they have programmatic control. That makes them more powerful, but more complex, and they require a separate `proc-macro = true` crate.

- [x] How is code tested in Rust? Where should you put tests and why?
    - Rust has built-in testing with the `#[test]` attribute and `cargo test` command.
    - Unit tests go in the same file as the code being tested, in a `tests` module with `#[cfg(test)]`. This keeps tests close to implementation, allows testing private functions, and is conditionally compiled only during testing.
    - Integration tests are put in a separate `tests/` directory at the project root. Each file is a separate crate that tests a library's public interface as external users would, catching API issues.
    - There are also documentation tests, which are written in doc comments and are automatically tested, this ensures examples in documentation stay correct.

- [x] Why does Rust have `&str` and `String` types? How do they differ? When should you use them?
    - Source: [FAQ What are the differences between the two different string types?](https://prev.rust-lang.org/en-US/faq.html#what-are-the-differences-between-str-and-string)
    - The `&str` is a primitive type implemented by the Rust language, while `String` is implemented in the standard library.
    - `String` is an owned buffer of UTF-8 bytes allocated on the heap. Mutable Strings can be modified, growing their capacity as needed. `&str` is a fixed-capacity "view" into a `String` allocated elsewhere, commonly on the heap, in the case of slices dereferenced from Strings, or in static memory, in the case of string literals. 

- [x] What are lifetimes? Which problems do they solve? Which benefits do they give?
    - Source: [FAQ Lifetimes](https://prev.rust-lang.org/en-US/faq.html#why-lifetimes)
    - Lifetimes are Rust's answer to the question of memory safety. They allow Rust to ensure memory safety without the performance costs of garbage collection. The `'a` syntax comes from the ML family of programming languages, where `'a` is used to indicate a generic type parameter.
    - All reference types have a lifetime, but most of the time you do not have to write it explicitly.
    - Oftentimes you can eliminate the references entirely by returning an owning type, it is a simpler approach, yet it often results in unnecessary allocations.
    - The major benefit of references with lifetimes is that they solve _null_ or _dangling_ pointers, as the only way to construct a value of type `&Foo` or `&mut Foo` is to specify an existing value of type `Foo` that the reference points to. The reference "borrows" the original value for a given region of code (the lifetime of the reference), and the value being borrowed from cannot be moved or destroyed for the duration of the borrow.

- [x] Is Rust an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance?
    - Source: [FAQ Design Patterns](https://prev.rust-lang.org/en-US/faq.html#is-rust-object-oriented), "trust me bro".
    - Rust is multi-paradigm. Many things you can do in OO languages you can do in Rust, but not everything, and not always using the same abstraction you're accustomed to.
    - There are ways of translating object-oriented concepts like multiple inheritance to Rust, but as Rust is not object-oriented the result of the translation may look substantially different from its appearance in an OO language.
    - Rust favors composition over inheritance and explicit trait implementations over implicit subtyping, making it more functional/data-oriented than classically object-oriented. This avoids classic inheritance problems like fragile base classes.
    - Most principles from SOLID/GRASP apply well:
        - single responsibility is naturally enforced by the module system;
        - open/closed architecture is achieved through traits and generics;
        - trait objects guarantee behavioral substitution;
        - traits are typically small and focused, so there is interface segregation;
        - dependency inversion with depending on traits and not concrete types.
    - GRASP patterns like Information Expert, Low Coupling, and High Cohesion align naturally with Rust's design.


## Material
- [x] [The Rust Book](https://doc.rust-lang.org/book)
- [x] [Rust FAQ](https://prev.rust-lang.org/faq.html)
- [ ] [Rust By Example](https://doc.rust-lang.org/rust-by-example)
- [x] [Rustlings](https://rustlings.cool)
- [ ] [The Cargo Book](https://doc.rust-lang.org/cargo)

_Additional_ articles, which may help to understand the above topic better:
- [x] [Ludwig Stecher: Rusts Module System Explained](https://aloso.github.io/2021/03/28/module-system.html)
    - With `pub(in path)`, visibility can also be limited to any other module: `pub(in crate::foo) struct Baz;`.
    - `pub(self)` is the default visibility.
    - Enum variants and variant fields are always public, so changing the variants or fields later is not backwards compatible.
    - Macros visibility rules are [complicated](https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing). 
        - The `#[macro_export]` attribute can be added to a macro, which exports it publicly at the crate root. This can be undesirable, however, if it's not supposed to be part of the public API; there is no equivalent of `pub(crate)` for macros.
        - One workaround for this is to put our macros in a module and annotate the module with `#[macro_use]`. The module should be the first module declaration in the crate root. This ensures that the macros can be used everywhere in our crate, but not outside of the crate. Not the most elegant solution, but it works.
- [x] [Brandon Smith: Three Kinds of Polymorphism in Rust](https://www.brandons.me/blog/polymorphism-in-rust)
- [x] [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust](https://www.thecodedmessage.com/posts/raii)
    - No Turing-complete programming language can completely prevent memory leaks.
    - **RAII** (Resource Acquisition Is Initialization) is a terrible name. **OBRM** (Ownership-Based Resource Management) is used in Rust sometimes, and is a much better name.
- [x] [Bradford Hovinen: Demystifying trait generics in Rust](https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust)
    - Associated types are sometimes called a trait's output types and generic parameters the trait's input types. The user of a trait specifies arguments for its generic parameters. An implementation of the trait specifies its associated types.
    - As a general rule, it's best to use the least powerful feature of a language to achieve your goal. Thus I would recommend using associated types by default. Only if you need the extra flexibility should you use generic type parameters.

- [ ] [HashRust: A guide to closures in Rust](https://hashrust.com/blog/a-guide-to-closures-in-rust)
- [ ] [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More](https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics)
- [ ] [cooscoos: &stress about &Strings](https://cooscoos.github.io/blog/stress-about-strings)

- [ ] [Jeremy Steward: C++ & Rust: Generics and Specialization](https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures)
- [ ] [Lukasz Uszko: Safe and Secure Coding in Rust: A Comparative Analysis of Rust and C/C++](https://luk6xff.github.io/other/safe_secure_rust_book/intro/index.html)
- [ ] [Georgios Antonopoulos: Rust vs Common C++ Bugs](https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust)
- [ ] [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust](https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust)
- [ ] [Clayton Ramsey: I built a garbage collector for a language that doesn't need one](https://claytonwramsey.github.io/2023/08/14/dumpster.html)

- [ ] [George He: Thinking in Rust: Ownership, Access, and Memory Safety](https://cocoindex.io/blogs/rust-ownership-access)
- [ ] [Chris Morgan: Rust ownership, the hard way](https://chrismorgan.info/blog/rust-ownership-the-hard-way)
- [ ] [Adolfo Ochagavía: You are holding it wrong](https://ochagavia.nl/blog/you-are-holding-it-wrong)
- [ ] [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker](https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad)
- [ ] [Sabrina Jewson: Why the "Null" Lifetime Does Not Exist](https://sabrinajewson.org/blog/null-lifetime)
- [ ] [Jeff Anderson: Generics Demystified Part 1](https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1)
- [ ] [Jeff Anderson: Generics Demystified Part 2](https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2)

## Notes
### [FAQ Rust compilation seems slow. Why is that?](https://prev.rust-lang.org/en-US/faq.html#why-is-rustc-slow)
C++'s compilation unit is the _file_, while Rust's is the _crate, composed of many files_. Thus, during development, modifying a single C++ file can result in much less recompilation than in Rust.

While Rust's preferred strategy of monomorphising generics (ala C++) produces fast code, it demands that significantly more code be generated than other translation strategies. Rust programmers can use **trait objects** to trade away this code bloat by using **dynamic dispatch** instead.

### [FAQ Does Rust do tail-call optimization?](https://prev.rust-lang.org/en-US/faq.html#does-rust-do-tail-call-optimization)
Rust has a keyword (`become`) reserved, though it is not clear yet whether it is technically possible, nor whether it will be implemented.

#### [FAQ Tail Call Optimisation in C](https://www.geeksforgeeks.org/c/tail-call-optimisation-in-c/)
The tail call is the type of function call where another function is called as the last action of the current function.

Tail Call Optimization (TCO) is a technique that eliminates the need for an additional stack frame to store the data of another function by reusing the current function's stack frame. This optimization technique is only possible for tail function calls.

This helps prevent stack overflow and can make recursive functions as memory-efficient as loops.

### [FAQ Does Rust have a runtime?](https://prev.rust-lang.org/en-US/faq.html#does-rust-have-a-runtime)
The Rust standard library additionally links to the C standard library. Rust code can be compiled **without** the standard library, in which case the runtime is roughly equivalent to C's.

### [FAQ How do I return a closure from a function?](https://prev.rust-lang.org/en-US/faq.html#how-do-i-return-a-closure-from-a-function)
To return a closure from a function, it must be a "move closure", meaning that the closure is declared with the `move` keyword. This gives the closure its own copy of the captured variables, independent of its parent stack frame. Otherwise, returning a closure would be unsafe, as it would allow access to variables that are no longer valid; put another way: it would allow reading potentially invalid memory. The closure must also be wrapped in a `Box`, so that it is allocated on the heap.

### [FAQ What's the difference between a function and a closure that doesn't capture any variables?](https://prev.rust-lang.org/en-US/faq.html#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture)
Functions are a built-in primitive of the language, while closures are essentially syntactic sugar for one of three traits: `Fn`, `FnMut`, and `FnOnce`. When you make a closure, the Rust compiler automatically creates a struct implementing the appropriate trait of those three and containing the captured environment variables as members, and makes it so the struct can be called as a function. Bare functions can not capture an environment.

The big difference between these traits is how they take the `self` parameter. `Fn` takes `&self`, `FnMut` takes `&mut self`, and `FnOnce` takes `self`.

Even if a closure does not capture any environment variables, it is represented at runtime as two pointers, the same as any other closure.

### Strings
Rust strings are UTF-8 encoded. A single visual character in UTF-8 is not necessarily a single byte as it would be in an ASCII-encoded string. Each byte is called a "code unit" (in UTF-16, code units are 2 bytes; in UTF-32 they are 4 bytes). "Code points" are composed of one or more code units, and combine in "grapheme clusters" which most closely approximate characters.
