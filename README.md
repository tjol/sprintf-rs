# lformat

**a clone of Lua `string.format` in Rust based on C `sprintf`**

Usage example:

```rust
use lformat::format;
let s = format("%d + %d = %d\n", &[&3, &9, &(3+9)]).unwrap();
assert_eq!(s, "3 + 9 = 12\n");
```
