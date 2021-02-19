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
amount of space for the resulting thing on the stack, which can't be done in a
generic way. Another alternative is to use `Box<...>` to pass a pointer to some
value on the heap.

## Printing and formatting


