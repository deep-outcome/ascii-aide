# husky-lib-core
ASCII table with views into it. Core lib component of [husky](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/husky). No std & no alloc.

```rust
use husky_lib_core::ranges::{ranges, Ranges};

let rs = ranges(Ranges::Capital);
assert_eq!('A', rs[0].clone().min().unwrap() as u8 as char);
assert_eq!('Z', rs[0].clone().max().unwrap() as u8 as char);
```
