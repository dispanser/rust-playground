# Rust: baby steps to world domination

Some low-tech project management tooling, aka markdown with checkboxes.

Overall goal: get somewhat fluent with rust, up to a level that allows me to
understand the major differences to other programming languages and being able
to write code to some level of proficiency. This requires:

- basic syntax: 
-- [X] functions
-- [X] standard types
-- [ ] defining own types, structs
-- [ ] pattern matching
-- [ ] implementing traits (and roll your own)
-- [ ] collections: `Vec`, iterators, slices etc
-- [ ] higher-order functions: flatmap that shit
-- [ ] algebraic data types
-- [ ] ... (tbd)
- testing
-- [X] unit tests
-- [X] quickcheck / property-based: using proptests
-- [ ] doctests: rustdoc code examples that compile and assert, whoo!
-- [ ] fuzzing: generating examples that try to hit under-covered code paths
-- [ ] see [LogRocket blog](https://blog.logrocket.com/how-to-organize-your-rust-tests/)
- tooling
-- [ ] intellij: setup
-- [ ] intellij: debug
-- [ ] vim support: code completion et al
-- [ ] benchmarking: [criterion](https://docs.rs/criterion/0.3.4/criterion/)
- standard library
-- [ ] handling files
-- [ ] handling network
-- [ ] ... (tbd)
- understanding the memory concept
-- [ ] references, `&mut`, `Box`, ...
-- [ ] grasp of the ownership model
-- [ ] ... (unknown unknowns)
- eco system
-- [ ] roll your own rest API
-- [ ] dive into delta-rs, parquet, arrow
-- [ ] ballista ftw!
- actor systems
-- [Actix](https://github.com/actix/actix)
-- [Racer](https://riker.rs/actors/)
-- performance: compare to akka

We start simple, and write code for everything we try. The list above is just
a bunch of starting points for exploration. Also, the idea is that do the most
simple thing that allows crossing of some goal.

