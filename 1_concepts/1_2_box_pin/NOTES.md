# Box
## [std::boxed](https://doc.rust-lang.org/std/boxed/index.html)
Boxes ensure that they never allocate more than `isize::MAX` bytes.

[Memory layout](https://doc.rust-lang.org/std/boxed/index.html#memory-layout) explains how to use `Box` with a raw pointer to allocated memory.

So long as `T: Sized`, a `Box<T>` is guaranteed to be represented as a single pointer and is also ABI-compatible with C pointers (i.e. the C type `T*`).

In general, the best practice is to only use `Box<T>` for pointers that originated from the global allocator.

**Important.** At least at present, you should avoid using `Box<T>` types for functions that are defined in C but invoked from Rust. In those cases, you should directly mirror the C types as closely as possible. Using types like `Box<T>` where the C definition is just using `T*` can lead to undefined behavior, as described in [rust-lang/unsafe-code-guidelines#198](https://github.com/rust-lang/unsafe-code-guidelines/issues/198).

### Editions
A special case exists for the implementation of `IntoIterator` for arrays on the Rust 2021 edition, as documented [here](https://doc.rust-lang.org/std/primitive.array.html "primitive array"). Unfortunately, it was later found that a similar workaround should be added for boxed slices, and this was applied in the 2024 edition.

Specifically, `IntoIterator` is implemented for `Box<[T]>` on all editions, but specific calls to `into_iter()` for boxed slices will defer to the slice implementation on editions before 2024:

Rust 2015, 2018, and 2021:
```rs
let boxed_slice: Box<[i32]> = vec![0; 3].into_boxed_slice();

// This creates a slice iterator, producing references to each value.
for item in boxed_slice.into_iter().enumerate() {
    let (i, x): (usize, &i32) = item;
    println!("boxed_slice[{i}] = {x}");
}

// The `boxed_slice_into_iter` lint suggests this change for future compatibility:
for item in boxed_slice.iter().enumerate() {
    let (i, x): (usize, &i32) = item;
    println!("boxed_slice[{i}] = {x}");
}

// You can explicitly iterate a boxed slice by value using `IntoIterator::into_iter`
for item in IntoIterator::into_iter(boxed_slice).enumerate() {
    let (i, x): (usize, i32) = item;
    println!("boxed_slice[{i}] = {x}");
}
```

## [Primitive Type slice](https://doc.rust-lang.org/std/primitive.slice.html)
It is possible to slice empty subranges of slices by using empty ranges (including `slice.len()..slice.len()`):
```rs
let x = [1, 2, 3];
let empty = &x[0..0];   // subslice before the first element
assert_eq!(empty, &[]);
let empty = &x[..0];    // same as &x[0..0]
assert_eq!(empty, &[]);
let empty = &x[1..1];   // empty subslice in the middle
assert_eq!(empty, &[]);
```

It is not allowed to use subranges that start with lower bound bigger than `slice.len()`:
```rs
let x = vec![1, 2, 3];
let _ = &x[4..4];
```

As slices store the length of the sequence they refer to, they have twice the size of pointers to [`Sized`](https://doc.rust-lang.org/std/marker/trait.Sized.html) types.


## [Amos: What's in the box?](https://fasterthanli.me/articles/whats-in-the-box)
GDB disables [Address Space Layout Randomization (ASLR)](https://en.wikipedia.org/wiki/Address_space_layout_randomization) by default.

Calling a function “allocates on the stack”. That’s _why_ the list we look at when we try to find where an error occurred is called a “stack trace” (or a “call stack”).

### Go
Go has its own stack. And its own heap. And everything is garbage-collected. And also, that makes goroutines cheap, and they can [adjust their stack size dynamically](https://dave.cheney.net/2013/06/02/why-is-a-goroutines-stack-infinite), and it also complicates [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface) a bunch.

The garbage collector in Go doesn’t really zero out memory blocks when it frees them, it just "marks them as free", and doesn’t change anything about the _contents_ of the memory.
`GODEBUG=clobberfree=1` can be used to cause the garbage collector to clobber the memory content of an object with bad content when it frees the object.

Go `string` values are actually structs, with a `Data` field, that points somewhere in memory. The structs themselves, of type `reflect.StringHeader`, have copy semantics, so `s2 := s1` creates a new `StringHeader`, pointing to the same area in memory.

The area in memory to which a `StringHeader` can point to can be in two different regions: "static data" mapped directly from the executable file, for string constants, or "that big block Go allocates", where the GC heap lives.

In fact, the Go compiler tries _very hard_ to stack-allocate as much as possible, and only uses the heap when it has no other choice.


### Rust

[String](https://doc.rust-lang.org/stable/std/string/struct.String.html) in Rust does not implement the [Copy](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html) trait, so it has "move semantics", unlike `string` in Go which has "copy semantics".

With reference-counting, whenever we clone an `Arc`, a counter is incremented. And whenever an `Arc` falls out of scope (is “dropped”), that counter is decremented. And that already is “some work” — especially with an `Arc`, because the counter is atomic (that’s what the `A` stands for).

Whenever we _hold_ a value, we must know its size. And in Rust, that property is indicated by the marker trait [Sized](https://doc.rust-lang.org/stable/std/marker/trait.Sized.html).

Because the `Sized` constraint is implicit, there exists a way to relax it, and it’s spelled `?Sized`.

A `Box<std::io::Error>` is 8 bytes. A `Box<dyn std::error::Error>` is 16 bytes: One pointer for the value, and one for the type.
It’s roughly the same as interfaces in Go, except the second pointer in a `Box<dyn T>`, whose real name is a “boxed trait object”, is not a pointer to “the concrete type”. It’s a pointer to the “virtual table that corresponds to the implementation of the interface for the concrete type”.

There is no safe way to downcast from a `Box<dyn T>` to a concrete type `U`, without using [Any](https://doc.rust-lang.org/stable/std/any/trait.Any.html), which is made explicitly for that purpose.

Closures are the "It who cannot be named". `closure [closure@src/main.rs:2:23: 4:6]` is not the name of a type, you can't name it. So you either box it (which forces a heap allocation):
```rs
fn get_closure() -> Box<dyn Fn()> {
    Box::new(|| { println!("hello from the closure side"); })
}
```
_or_ we can use `impl Trait` syntax:
```rs
fn get_closure() -> impl Fn() {
    || { println!("hello from the closure side"); }
}
```
Using `std::mem::size_of_val`, we can print the size of that closure -- 0. The size of the closure that captures something:
```rs
fn get_closure() -> impl Fn() {
    let val = 27_u128;
    move || { println!("hello from the closure side, val is {}", val); }
}
```
is `16`, the size of the captured value.




# Pin

## [Official `std::pin` docs](https://doc.rust-lang.org/std/pin)
Note that as long as you don’t use `unsafe`, it’s impossible to create or misuse a pinned value in a way that is unsound.

All values in Rust are trivially _moveable_. This means that the address at which a value is located is not necessarily stable in between borrows.

Common smart-pointer types such as `Box<T>` and `&mut T` also allow _moving_ the underlying _value_ they point at: you can move out of a `Box<T>`, or you can use `mem::replace` to move a `T` out of a `&mut T`.

We say that a value has been _pinned_ when it has been put into a state where it is guaranteed to remain _located at the same place in memory_ from the time it is pinned until its `drop` is called.

Notice that the thing wrapped by `Pin` is not the value which we want to pin itself, but rather a pointer to that value! A `Pin<Ptr>` does not pin the `Ptr`; instead, it pins the pointer’s _**pointee** value_.
A `Pin<Ptr>` where `Ptr: Deref` is a “`Ptr`-style pinning pointer” to a pinned `Ptr::Target` – so, a `Pin<Box<T>>` is an owned, pinning pointer to a pinned `T`, and a `Pin<Rc<T>>` is a reference-counted, pinning pointer to a pinned `T`.

A `Pin<&mut T>` makes it impossible to obtain the wrapped `&mut T` safely because through that `&mut T` it would be possible to _move_ the underlying value out of the pointer with `mem::replace`, etc.

The vast majority of Rust types have no address-sensitive states. These types implement the `Unpin` auto-trait, which cancels the restrictive effects of `Pin` when the _pointee_ type `T` is `Unpin`. When `T: Unpin`, `Pin<Box<T>>` functions identically to a non-pinning `Box<T>`; similarly, `Pin<&mut T>` would impose no additional restrictions above a regular `&mut T`.

The compiler is free to take the conservative stance of marking types as `Unpin` so long as all of the types that compose its fields are also `Unpin`.

If you really need to pin a value of a foreign or built-in type that implements `Unpin`, you’ll need to create your own wrapper type around the `Unpin` type you want to pin and then opt-out of `Unpin` using `PhantomPinned`:
```rs
struct AddrTracker {
    prev_addr: Option<usize>,
    _pin: PhantomPinned,
}

impl AddrTracker {
    fn check_for_move(self: Pin<&mut Self>) {
        let current_addr = &*self as *const Self as usize;
        match self.prev_addr {
            None => {
                // SAFETY: we do not move out of self
                let self_data_mut = unsafe { self.get_unchecked_mut() };
                self_data_mut.prev_addr = Some(current_addr);
            },
            Some(prev_addr) => assert_eq!(prev_addr, current_addr),
        }
    }
}
```

Exposing access to the inner field which you want to remain pinned must then be carefully considered as well! Remember, exposing a method that gives access to a `Pin<&mut InnerT>` where `InnerT: Unpin` would allow safe code to trivially move the inner value out of that pinning pointer, which is precisely what you’re seeking to prevent! Exposing a field of a pinned value through a pinning pointer is called “projecting” a pin, and the more general case of deciding in which cases a pin should be able to be projected or not is called “structural pinning.” We will go into more detail about this [here](https://doc.rust-lang.org/stable/std/pin/index.html#projections-and-structural-pinning "mod std::pin").

If your type is [`#[repr(packed)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprpacked), the compiler will automatically move fields around to be able to drop them. It might even do that for fields that happen to be sufficiently aligned. As a consequence, you cannot use pinning with a `#[repr(packed)]` type.

### [`Drop` guarantee](https://doc.rust-lang.org/stable/std/pin/index.html#subtle-details-and-the-drop-guarantee "mod std::pin")
The purpose of pinning is not _just_ to prevent a value from being _moved_ , but more generally to be able to rely on the pinned value _remaining valid **at a specific place**_ in memory.

From the moment a value is pinned by constructing a `Pin`ning pointer to it, that value must _remain, **valid**_ , at that same address in memory, _until its `drop` handler is called._ This point is subtle but required for intrusive data structures to be implemented soundly.

It is crucial to remember that `Pin`ned data _must_ be `drop`ped before it is invalidated; not just to prevent memory leaks, but as a matter of soundness.

### [Projections and Structural Pinning](https://doc.rust-lang.org/stable/std/pin/#projections-and-structural-pinning)
ugh...

As the author of a data structure, you get to decide for each field whether pinning “propagates” to this field or not. Pinning that propagates is also called “structural”, because it follows the structure of the type:

- Fields without structural pinning may have a projection method that turns `Pin<&mut Struct>` into `&mut Field`.
- The other option is to decide that pinning is “structural” for `field`, meaning that if the struct is pinned then so is the field.

In the standard library, pointer types generally do not have structural pinning, and thus they do not offer pinning projections. This is why `Box<T>: Unpin` holds for all `T`. It makes sense to do this for pointer types, because moving the `Box<T>` does not actually move the `T`: the `Box<T>` can be freely movable (aka `Unpin`) even if the `T` is not. In fact, even `Pin<Box<T>>` and `Pin<&mut T>` are always `Unpin` themselves, for the same reason: their contents (the `T`) are pinned, but the pointers themselves can be moved without moving the pinned data. For both `Box<T>` and `Pin<Box<T>>`, whether the content is pinned is entirely independent of whether the pointer is pinned, meaning pinning is _not_ structural.

When implementing a `Future` combinator, you will usually need structural pinning for the nested futures, as you need to get pinning (`Pin`-wrapped) references to them to call `poll`. But if your combinator contains any other data that does not need to be pinned, you can make those fields not structural and hence freely access them with a mutable reference even when you just have `Pin<&mut Self>` (such as in your own `poll` implementation).

## std::pin::pin macro

The local pinning performed by this macro is usually dubbed “stack”-pinning. Unlike [`Box::pin`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.pin), this does not create a new heap allocation. The element might still end up on the heap however.

Precisely because a value is pinned to local storage, the resulting `Pin<&mut T>` reference ends up borrowing a local tied to that block: it can’t escape it. This makes `pin!` **unsuitable to pin values when intending to _return_ them**. Instead, the value is expected to be passed around _unpinned_ until the point where it is to be consumed, where it is then useful and even sensible to pin the value locally using `pin!`.

By virtue of not requiring an allocator, `pin!` is the main non-`unsafe` `#![no_std]`-compatible `Pin` constructor.

## std::pin::Pin
If the pointee value’s type does not implement `Unpin`, then Rust will not let us use the `Pin::new` function directly.

The `Future` trait requires all calls to `poll` to use a `self: Pin<&mut Self>` parameter instead of the usual `&mut self`. Therefore, when manually polling a future, you will need to pin it first.

The simplest and most flexible way to pin a value that does not implement `Unpin` is to put that value inside a `Box` and then turn that `Box` into a “pinning `Box`” by wrapping it in a `Pin`. You can do both of these in a single step using `Box::pin`.

If you have a value which is already boxed, for example a `Box<dyn Future>`, you can pin that value in-place at its current memory address using `Box::into_pin`.

