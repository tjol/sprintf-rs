# sprintf-rs - a clone of C s(n)printf in Rust

This crate was created out of a desire to provide C printf-style formatting
in a WASM program, where there is no libc.

**Note:** *You're probably better off using standard Rust string formatting
instead of thie create unless you specificaly need printf compatibility.*

This crate implements a dynamically type-checked function `vsprintf` and macro
`sprintf!`.

Usage example:

```rust
use sprintf::sprintf;
let s = sprintf!("%d + %d = %d\n", 3, 9, 3+9).unwrap();
assert_eq!(s, "3 + 9 = 12\n");
```

`libc` is a dev devependency as it is used in the tests to compare results.
