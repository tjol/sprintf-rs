# sprintf-rs

**a clone of C sprintf in Rust**

This crate was created out of a desire to provide C printf-style formatting
in a WASM program, where there is no libc.

**Note:** _You're probably better off using standard Rust string formatting
instead of this crate unless you specificaly need printf compatibility._

This crate implements a dynamically type-checked function `vsprintf` and macro
`sprintf!`.

Usage example:

```rust
use sprintf::sprintf;
let s = sprintf!("%d + %d = %d\n", 3, 9, 3+9).unwrap();
assert_eq!(s, "3 + 9 = 12\n");
```

`libc` is a dev dependency as it is used in the tests to compare results. This
crate depends on `std` by default, but the default features can be disabled and
the `no_std` feature can be enabled as an alternative.
