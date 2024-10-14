# huski-lib-core
ASCII table with views into it. Core lib component of [huski](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/huski). No std & no alloc.

```rust
use huski_lib_core::ranges::{ranges, Ranges};

let rs = ranges(Ranges::Capital);
assert_eq!('A', rs[0].clone().min().unwrap() as u8 as char);
assert_eq!('Z', rs[0].clone().max().unwrap() as u8 as char);
```
