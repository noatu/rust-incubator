`smart-default` crate allows deriving Clone with customized default values.

## [move semantics](https://stackoverflow.com/questions/30288782/what-are-move-semantics-in-rust/30290070#30290070)
Rust is a language built for speed, and there are numerous optimizations passes at play here which will depend on the compiler used.

When calling a function in theory you "move" the object into the function stack frame, however in practice if the object is large the rustc compiler might switch to passing a pointer instead.

Another situation is returning from a function, but even then the compiler might apply "return value optimization" and build directly in the caller's stack frame -- that is, the caller passes a pointer into which to write the return value, which is used without intermediary storage.

The ownership/borrowing constraints of Rust enable optimizations that are difficult to reach in C++ (which also has RVO but cannot apply it in as many cases).

So:
- Moving large objects is inefficient, but there are a number of optimizations at play that might elide the move altogether.
- Moving involves a `memcpy` of `std::mem::size_of::<T>()` bytes, so moving a large `String` is efficient because it only copies a couple bytes whatever the size of the allocated buffer they hold onto.

## [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)
For a generic struct, `#[derive]` implements `Clone` conditionally by adding bound `Clone` on generic parameters.
```rs
// `derive` implements Clone for Reading<T> when T is Clone.
#[derive(Clone)]
struct Reading<T> {
    frequency: T,
}
```

This can be unnecessary:
```rs
#[derive(Copy, Clone)]
struct Generate<T>(fn() -> T);
```
results in:
```rs
impl<T: Copy> Copy for Generate<T> { }

impl<T: Clone> Clone for Generate<T> {
    fn clone(&self) -> Generate<T> {
        Generate(Clone::clone(&self.0))
    }
}
```

The bounds are unnecessary because clearly the function itself should be copy- and cloneable even if its return type is not, this can be fixed with manual implementation:
```rs
struct Generate<T>(fn() -> T);

impl<T> Copy for Generate<T> {}

impl<T> Clone for Generate<T> {
    fn clone(&self) -> Self {
        *self
    }
}

struct NotCloneable;

fn generate_not_cloneable() -> NotCloneable {
    NotCloneable
}

Generate(generate_not_cloneable).clone();
```

### The following types also implement `Clone`:
- Function item types (i.e., the distinct types defined for each function)
- Function pointer types (e.g., `fn() -> i32`)
- Closure types, if they capture no value from the environment or if all such captured values implement `Clone` themselves. Note that variables captured by shared reference always implement `Clone` (even if the referent doesn’t), while variables captured by mutable reference never implement `Clone`.

### fn [clone_from](https://doc.rust-lang.org/std/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`.

`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a` to avoid unnecessary allocations.

## Copy
By default, variable bindings have ‘move semantics.’. However, if a type implements Copy, it instead has ‘copy semantics’.

The `derive` strategy will also place a `Copy` bound on type parameters:

```rs
#[derive(Clone)]
struct MyStruct<T>(T);

impl<T: Copy> Copy for MyStruct<T> { }
```

This isn’t always desired. For example, shared references (`&T`) can be copied regardless of whether `T` is `Copy`:
```rs
#[derive(Copy, Clone)]
struct PointListWrapper<'a> {
    point_list_ref: &'a PointList,
}
```
Likewise, a generic struct containing markers such as [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html "struct std::marker::PhantomData") could potentially be duplicated with a bit-wise copy.

## [Moves, copies and clones in Rust](https://hashrust.com/blog/moves-copies-and-clones-in-rust)

C++ makes a deep copy when a vector is assigned to another variable.
