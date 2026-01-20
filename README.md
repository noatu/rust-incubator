# Rust Incubator

## ⏱️ Incubator Schedule

- [x] [0. Become familiar with Rust basics][Step 0] (3 days)
- [ ] [1. Concepts][Step 1] (2 days, after all sub-steps)
    - [x] [1.1. Default values, cloning and copying][Step 1.1] (1 day)
    - [ ] [1.2. Boxing and pinning][Step 1.2] (1 day)
    - [ ] [1.3. Shared ownership and interior mutability][Step 1.3] (1 day)
    - [ ] [1.4. Clone-on-write][Step 1.4] (1 day)
    - [ ] [1.5. Conversions, casting and dereferencing][Step 1.5] (1 day)
    - [ ] [1.6. Static and dynamic dispatch][Step 1.6] (1 day)
    - [ ] [1.7. `Sized` and `?Sized` types][Step 1.7] (1 day)
    - [ ] [1.8. Thread safety][Step 1.8] (1 day)
    - [ ] [1.9. Phantom types][Step 1.9] (1 day)
- [ ] [2. Idioms][Step 2] (2 days, after all sub-steps)
    - [ ] [2.1. Rich types ensure correctness][Step 2.1] (1 day)
    - [ ] [2.2. Swapping values with `mem::replace`][Step 2.2] (1 day)
    - [ ] [2.3. Bound behavior, not data][Step 2.3] (1 day)
    - [ ] [2.4. Abstract type in, concrete type out][Step 2.4] (1 day)
    - [ ] [2.5. Exhaustivity][Step 2.5] (1 day)
    - [ ] [2.6. Sealing][Step 2.6] (1 day)
- [ ] [3. Common ecosystem][Step 3] (2 days, after all sub-steps)
    - [ ] [3.1. Testing and mocking][Step 3.1] (1 day)
    - [ ] [3.2. Declarative and procedural macros][Step 3.2] (1 day)
    - [ ] [3.3. Date and time][Step 3.3] (1 day)
    - [ ] [3.4. Regular expressions and custom parsers][Step 3.4] (1 day)
    - [ ] [3.5. Collections and iterators][Step 3.5] (1 day)
    - [ ] [3.6. Serialization and deserialization][Step 3.6] (1 day)
    - [ ] [3.7. Randomness and cryptography][Step 3.7] (1 day)
    - [ ] [3.8. Logging and tracing][Step 3.8] (1 day)
    - [ ] [3.9. Command-line arguments, environment variables and configs][Step 3.9] (1 day)
    - [ ] [3.10. Multithreading and parallelism][Step 3.10] (1 day)
    - [ ] [3.11. Async I/O, futures and actors][Step 3.11] (2 days)
- [ ] [4. Backend ecosystem][Step 4] (3 days, after all sub-steps)
    - [ ] [4.1. Databases, connection pools and ORMs][Step 4.1] (1 day)
    - [ ] [4.2. HTTP servers and clients][Step 4.2] (1 day)
    - [ ] [4.3. API servers, clients and tools][Step 4.3] (1 day)


## 🎓 My Learning So Far

### ✅ Completed
#### 📚 Books
- [Rust Book](https://doc.rust-lang.org/book): Rust basics.

#### 🤺 Exercises
- [Macro Kata](https://tfpk.github.io/macrokata): a set of exercises which you can use to learn how to write macros.
- [Rustlings](https://rustlings.cool): a collection of small exercises to get you used to reading and writing Rust code.
- (did not do the `seq` macro) [Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop): a selection of projects designed to learn to write Rust procedural macros — Rust code that generates Rust code.

#### 📼 Videos
- [Rust Data Modelling Without Classes](https://www.youtube.com/watch?v=z-0-bbc80JM): nice video by No Boilerplate about state machines.
- ["Making Impossible States Impossible" by Richard Feldman](https://www.youtube.com/watch?v=IcgmSRJHu_8): ELM conference presentation about modeling types so that erroneous states are impossible to represent.
- [Building a simple text editor with iced, a cross-platform GUI library for Rust](https://www.youtube.com/watch?v=gcBJ7cPSALo).

#### 📝 Miscellaneous
- [Rust FAQ](https://prev.rust-lang.org/faq.html): answers common questions about Rust.
- [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern): utilizing Rust's type system to have a compile-time verified state machines.
- [How to apply hexagonal architecture to Rust](https://web.archive.org/web/20250725142154/https://www.barrage.net/blog/technology/how-to-apply-hexagonal-architecture-to-rust)

### 🤸 Doing
- This incubator

### 🚀 Started
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/): Should you wish a long and happy career of writing Rust programs, you should turn back now and forget you ever saw this book. It is not necessary.
- [Cargo Book](https://doc.rust-lang.org/cargo): a guide to Cargo, Rust's build tool and dependency manager.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines): a set of recommendations on how to design and present APIs for Rust.


### 🕵️ To Do
- [Rust Quiz](https://github.com/dtolnay/rust-quiz) for medium to hard Rust questions with explanations.
- [LifetimeKata](https://tfpk.github.io/lifetimekata/index.html): a set of exercises which you can use to improve your understanding of lifetimes in Rust.
- [High Assurance Rust](https://highassurance.rs/landing.html): developing secure and robust software.

#### 😕 Maybe 
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/): a pretty old book, but seems nice.

#### ❓ what are those
- [Rust By Example] teaches you Rust basics using editable examples.
- [Rust Reference] is not a formal spec, but is more detailed and comprehensive than the [Rust Book].
- [Cheats.rs] and [Rust SVG Cheatsheet] for quick reference.
- [Rust Edition Guide] for considering the improvements in [Rust 2018] and [Rust 2021].
- [Rust std lib] documentation.
- [Rustdoc Book] is a guide to the `rustdoc` documentation tool.
- [Rust Cookbook] is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using crates from the Rust ecosystem.
- [Rust Design Patterns] is an open source repository of Rust design patterns and idioms.
- [Effective Rust] is a collection of guidelines that had been learned from real world experience of creating software in Rust.
- [Rust Playground] allows sharing and checking runnable Rust code snippets online.
- [Awesome Rust] is a curated list of Rust code and resources.
- [This Week in Rust] represents handpicked and subscribable Rust weekly updates.
- [Baby Steps] blog by [Nicholas Matsakis](https://github.com/nikomatsakis) shares useful Rust patterns, ideas and design decisions.
- [Learning Material for Idiomatic Rust] is a curated list of resources to help you write ergonomic and idiomatic Rust code.


<!-- Helix macro:   gef:ll<S-D>d<S-F>:hvgs*%s<ret>a(<esc>pgexd   -->

[Step 0]: 0_basics
[Step 1]: 1_concepts
[Step 1.1]: 1_concepts/1_1_default_clone_copy
[Step 1.2]: 1_concepts/1_2_box_pin
[Step 1.3]: 1_concepts/1_3_rc_cell
[Step 1.4]: 1_concepts/1_4_cow
[Step 1.5]: 1_concepts/1_5_convert_cast_deref
[Step 1.6]: 1_concepts/1_6_dispatch
[Step 1.7]: 1_concepts/1_7_sized
[Step 1.8]: 1_concepts/1_8_thread_safety
[Step 1.9]: 1_concepts/1_9_phantom
[Step 2]: 2_idioms
[Step 2.1]: 2_idioms/2_1_type_safety
[Step 2.2]: 2_idioms/2_2_mem_replace
[Step 2.3]: 2_idioms/2_3_bound_impl
[Step 2.4]: 2_idioms/2_4_generic_in_type_out
[Step 2.5]: 2_idioms/2_5_exhaustivity
[Step 2.6]: 2_idioms/2_6_sealing
[Step 3]: 3_ecosystem
[Step 3.1]: 3_ecosystem/3_1_testing
[Step 3.2]: 3_ecosystem/3_2_macro
[Step 3.3]: 3_ecosystem/3_3_date_time
[Step 3.4]: 3_ecosystem/3_4_regex_parsing
[Step 3.5]: 3_ecosystem/3_5_collections
[Step 3.6]: 3_ecosystem/3_6_serde
[Step 3.7]: 3_ecosystem/3_7_rand_crypto
[Step 3.8]: 3_ecosystem/3_8_log
[Step 3.9]: 3_ecosystem/3_9_cmd_env_conf
[Step 3.10]: 3_ecosystem/3_10_threads
[Step 3.11]: 3_ecosystem/3_11_async
[Step 4]: 4_backend
[Step 4.1]: 4_backend/4_1_db
[Step 4.2]: 4_backend/4_2_http
[Step 4.3]: 4_backend/4_3_api

[Awesome Rust]: https://github.com/rust-unofficial/awesome-rust
[Baby Steps]: http://smallcultfollowing.com/babysteps
[Cargo]: https://github.com/rust-lang/cargo
[Cheats.rs]: https://cheats.rs
[CLion]: https://www.jetbrains.com/clion
[Clippy]: https://github.com/rust-lang/rust-clippy
[Effective Rust]: https://www.lurklurk.org/effective-rust
[GitHub repository]: https://help.github.com/articles/github-glossary/#repository
[Learning Material for Idiomatic Rust]: https://corrode.dev/blog/idiomatic-rust-resources
[PR]: https://help.github.com/articles/github-glossary/#pull-request
[RSS subscription]: https://github.com/instrumentisto/rust-incubator/commits/main.atom
[Rust]: https://www.rust-lang.org
[Rust 2018]: https://doc.rust-lang.org/edition-guide/rust-2018/index.html
[Rust 2021]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust Design Patterns]: https://rust-unofficial.github.io/patterns
[Rust Edition Guide]: https://doc.rust-lang.org/edition-guide
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[Rust on Exercism]: https://exercism.org/tracks/rust/exercises
[Rust Playground]: https://play.rust-lang.org
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust std lib]: https://doc.rust-lang.org/std
[Rust SVG Cheatsheet]: https://web.archive.org/web/20241001012119/https://www.breakdown-notes.com/make/load/rust_cs_canvas/true
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[rustfmt]: https://github.com/rust-lang/rustfmt
[rustup]: https://rustup.rs
[This Week in Rust]: https://this-week-in-rust.org

[1]: https://github.com/instrumentisto/rust-incubator/generate
[2]: https://github.com/instrumentisto/rust-incubator/subscription
[11]: https://help.github.com/en/articles/creating-a-repository-from-a-template
[13]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
