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

Closures are "It who cannot be named". `closure `[closure@src/main.rs:2:23: 4:6]` is not the name of a type, you can't name it. So you either box it (which forces a heap allocation):
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
Using `std::mem::size_of_val`, we can print the size of that closure.
