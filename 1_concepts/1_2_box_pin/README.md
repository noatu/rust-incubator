Step 1.2: Boxing and pinning
============================

__Estimated time__: 1 day


## Boxing

[`Box`] is a pointer that owns heap-allocated data. This is the most common and simplest form of [heap] allocation in [Rust].

It's more idiomatic to use references (`&T`/`&mut T`) for pointing to the data, however they often come with lifetime complexity. [`Box`] allows to avoid this complexity at the cost of heap allocation.

[`Box`] is also a way to go if an owned [slice] is needed, but is not intended to be resized. For example, `Box<str>`/`Box<[T]>` are often used instead of `String`/`Vec<T>` in such cases.

To better understand [`Box`]'s purpose, design, limitations, and use cases, read through:
- [x] [Rust Book: 15.1. Using Box to Point to Data on the Heap][1]
- [x] [Official `std::boxed` docs][`std::boxed`]
- [x] [Amos: What's in the box?][3]
- [x] [Mahdi Dibaiee: What is `Box<str>` and how is it different from `String` in Rust?][8]

## Pinning

It is sometimes useful to have objects that are guaranteed to not move, in the sense that their placement in memory does not change, and can thus be relied upon. A prime example of such a scenario would be building self-referential structs, since moving an object with pointers to itself would invalidate them, which could cause undefined behavior.

[`Pin<P>`][`Pin`] ensures that the pointee of any pointer type `P` has a stable location in memory, meaning it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped. We say that the pointee is "pinned".

However, many types are always freely movable, even when pinned, because they do not rely on having a stable address. This includes all the basic types (like `bool`, `i32`, references) as well as types consisting solely of these types. Types that do not care about pinning implement the [`Unpin`] marker trait, which cancels the effect of [`Pin`]. For `T: Unpin`, `Pin<Box<T>>` and `Box<T>` function identically, as do `Pin<&mut T>` and `&mut T`.

Note, that pinning and [`Unpin`] only affect the pointed-to type `P::Target`, not the pointer type `P` itself that got wrapped in `Pin<P>`. For example, whether or not `Box<T>` is `Unpin` has no effect on the behavior of `Pin<Box<T>>` (here, `T` is the pointed-to type).

To better understand [`Pin`]'s purpose, design, limitations, and use cases, read through:
- [x] [Official `std::pin` docs][`std::pin`]
- [x] [Amos: Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
- [ ] [Reddit: Pinned objects ELI5?][2]
- [ ] [SoByte: Pin and Unpin in Rust][10]
- [ ] [Adam Chalmers: Pin, Unpin, and why Rust needs them][4]
- [ ] [Tamme Schichler: Pinning in plain English][5]
- [ ] [Yoshua Wuyts: Safe Pin Projections Through View Types][6]
- [ ] [Official `#[pin_project]` docs][7]
- [ ] [Alice Ryhl answers on "Pin tutorial are confusing me"][9]
- [ ] [Rust Forum: Why is it unsafe to pin a shared reference?][11]
- [ ] [Ohad Ravid: Put a Pin on That][12]
- [ ] [Razieh Behjati: Leaky Abstractions and a Rusty Pin][13]
- [ ] [Saoirse Shipwreckt: Pin][14]




## Task

- [x] For the following types: `Box<T>`, `Rc<T>`, `Vec<T>`, `String`, `&[u8]`, `T`.
   Implement the following traits:
   ```rust
   trait SayHi: fmt::Debug {
       fn say_hi(self: Pin<&Self>) {
           println!("Hi from {:?}", self)
       }
   }
   ```
   ```rust
   trait MutMeSomehow {
       fn mut_me_somehow(self: Pin<&mut Self>) {
           // Implementation must be meaningful, and
           // obviously call something requiring `&mut self`.
           // The point here is to practice dealing with
           // `Pin<&mut Self>` -> `&mut self` conversion
           // in different contexts, without introducing 
           // any `Unpin` trait bounds.
       }
   }
   ```

- [x] For the following structure:
   ```rust
   struct MeasurableFuture<Fut> {
       inner_future: Fut,
       started_at: Option<std::time::Instant>,
   }
   ```
   Provide a [`Future`] trait implementation, transparently polling the `inner_future`, and printing its execution time in nanoseconds once it's ready. Using `Fut: Unpin` trait bound (or similar) is not allowed. 




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What does "boxing" mean in Rust? How is it useful? When and why is it required?
    - Boxing refers to wrapping something in a `Box<T>` smart pointer, which provides the simplest form of heap allocation in Rust. The value inside is owned and cleared when the `Box` is dropped. It is useful for objects that are large or have an unknown size or type (`dyn` objects). So long as `T: Sized`, a `Box<T>` is guaranteed to be represented as a single pointer and is also ABI-compatible with C pointers. `Box<dyn Trait>` is usually 2 pointers, one for data and one for the vtable. Because the size is also known, it lets you create recursive types. Also, there are DSTs (dynamically sized types) such as `str` and `[T]`, which need a pointer wrapper to be used directly.
- What is [`Pin`] and why is it required? What guarantees does it provide? How does it fulfill them?
    - All values in Rust are trivially _moveable_. This means that the address at which a value is located is not necessarily stable in between borrows. Common smart-pointer types such as `Box<T>` and `&mut T` also allow _moving_ the underlying _value_ they point at: you can move out of a `Box<T>`, or you can use `mem::replace` to move a `T` out of a `&mut T`.
    - `Pin` exists to ensure the pointee (the value pointed at, `T` in `Box<T>`) has been put into a state where it is guaranteed to remain _located at the same place in memory_ from the time it is pinned until its `drop` is called.
    - A `Pin<&mut T>` makes it impossible to obtain the wrapped `&mut T` safely because through that `&mut T` it would be possible to _move_ the underlying value out of the pointer with `mem::replace`, etc.
- How does [`Unpin`] affect the [`Pin`]? What does it mean?
    - `Unpin` is a marker auto-trait. The compiler is free to take the conservative stance of marking types as `Unpin` so long as all of the types that compose its fields are also `Unpin`.
    - The vast majority of Rust types have no address-sensitive states. These types implement the `Unpin` auto-trait, which cancels the restrictive effects of `Pin` when the _pointee_ type `T` is `Unpin`. When `T: Unpin`, `Pin<Box<T>>` functions identically to a non-pinning `Box<T>`; similarly, `Pin<&mut T>` would impose no additional restrictions above a regular `&mut T`.
    - Also, if the pointee value’s type does not implement `Unpin`, then Rust will not let us use the `Pin::new` function directly.
- Is it allowed to move pinned data after the [`Pin`] dies? Why?
    - It is allowed but it is a bad idea.
    - It is crucial to remember that Pinned data must be dropped before it is invalidated; not just to prevent memory leaks, but as a matter of soundness.
    - Re-pinning after a move may cause UB because internal self-references now point to stale memory locations.
    - So, if you don't pin the value again you can move and use it.
- What is structural pinning? When should it be used and why?
    - The general case of deciding in which cases a pin should be able to be projected or not is called “structural pinning”.
        - Fields without structural pinning may have a projection method that turns `Pin<&mut Struct>` into `&mut Field`.
        - The other option is to decide that pinning is “structural” for `field`, meaning that if the struct is pinned then so is the field.
    - When implementing a `Future` combinator, you will usually need structural pinning for the nested futures, as you need to get pinning (`Pin`-wrapped) references to them to call `poll`. But if your combinator contains any other data that does not need to be pinned, you can make those fields not structural and hence freely access them with a mutable reference even when you just have `Pin<&mut Self>` (such as in your own `poll` implementation).
- What is [`Pin`] projection? Why does it exist? How is it used?
    - Exposing a field of a pinned value `Pin<&mut Struct>` through a pinning pointer `Pin<&mut Field>` is called “projecting” a pin, and the more general case of deciding in which cases a pin should be able to be projected or not is called “structural pinning.”
    - It exists because when you have a pinned struct, you often need to call methods on its fields that also require pinning. If a future wrapper needs to poll an inner future, `poll` requires `Pin<&mut T>`. You can't get that pin through safe code because the compiler doesn't know which fields should maintain the pinning guarantee.
    - Pin projection works through **unsafe** code that manually upholds the pinning invariant. Given `Pin<&mut Struct>`, you use `get_unchecked_mut()` to get `&mut Struct`, then `Pin::new_unchecked(&mut struct.field)` to create the pinned field reference. The safety requirement is that if the outer struct is pinned, the field must never move. That holds true as long as you don't provide any safe way to move it.
    - For fields that are `Unpin`, you can access them directly without repinning since they don't have the same constraints.
    - The `pin-project` crate automates this pattern safely through proc macros, eliminating the manual unsafe code.


[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`Pin`]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[`std::boxed`]: https://doc.rust-lang.org/std/boxed/index.html
[`std::pin`]: https://doc.rust-lang.org/std/pin/index.html
[`Unpin`]: https://doc.rust-lang.org/std/marker/trait.Unpin.html
[heap]: https://en.wikipedia.org/wiki/Memory_management#HEAP
[Rust]: https://www.rust-lang.org
[slice]: https://doc.rust-lang.org/std/primitive.slice.html

[1]: https://doc.rust-lang.org/book/ch15-01-box.html
[2]: https://www.reddit.com/r/rust/comments/9akmqv/pinned_objects_eli5
[3]: https://fasterthanli.me/articles/whats-in-the-box
[4]: https://blog.adamchalmers.com/pin-unpin
[5]: https://blog.schichler.dev/pinning-in-plain-english-ckwdq3pd0065zwks10raohh85
[6]: https://blog.yoshuawuyts.com/safe-pin-projections-through-view-types
[7]: https://docs.rs/pin-project/latest/pin_project/attr.pin_project.html
[8]: https://web.archive.org/web/20230605135444/https://mahdi.blog/rust-box-str-vs-string
[9]: https://users.rust-lang.org/t/pin-tutorial-are-confusing-me/91003/18
[10]: https://www.sobyte.net/post/2022-07/rust-pin-unpin
[11]: https://users.rust-lang.org/t/why-is-it-unsafe-to-pin-a-shared-reference/40309
[12]: https://ohadravid.github.io/posts/2023-07-put-a-pin-on-that
[13]: https://itnext.io/leaky-abstractions-and-a-rusty-pin-fbf3b84eea1f
[14]: https://without.boats/blog/pin
