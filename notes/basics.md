# Rust: A Primitive Primer

## Iterating / Filtering

```rust
    (2 ..= n)
        .filter(|i| is_prime2(i))
        .collect::<Vec<i32>>();
```

- `filter` returns some `Filter<i32>` in this case, which isn't a `Vec`
- `collect` kinda reminds me of java streams, where you `collect(toList())`
- note that we produce a collection, which may not be desired

```rust
pub fn primes2(n: i32) -> impl Iterator<Item = i32> {
    (2..=n)
        .filter(|i| is_prime2(i))
}
```

It's not as straightforward to return a trait as I would expect coming from
Scala, Java or Haskell. IIUC, the compiler must be able to reserve the correct
amount of space for the resulting thing on the stack, which can't be done in
a generic way. Another alternative is to use `Box<...>` to pass a pointer to
some value on the heap, paid by with one additional indirection.

## Modules, Visibility n stuff

Blog post [rust module system](http://www.sheshbabu.com/posts/rust-module-system/)
has some details, but here's the tldr;:

- a module is either a file in `src/`, e.g. `src/primes.rs` or a directory,
  e.g. `src/primes/mod.rs`. 
- this logic is applied recursively: `src/primes/sieve.rs` would contain a
  module `primes::sieve`.
- to make modules usable, you have to declare them in the "upper level":
-- `src/main.rs` could `mod primes;` to make `primes::..` available for call
-- `src/lib.rs` could do the same, but if the user of the library, e.g. `mybin`
   (see below), wants to use it, we have to make it public via `pub mod primes;`
- `src/bin/mybin.rs` can be called with `cargo run --bin mybin`, via path magic
- if `mybin` wants to access something which is 
- per default, everything is private
-- modules
-- functions
-- structs
-- struct members


