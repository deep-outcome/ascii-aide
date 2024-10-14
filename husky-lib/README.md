# husky-lib
Facility for working with ASCII table and its views into it. Lib component of [husky](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/husky).

See also [husky-lib-core](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/husky-lib-core).


 ```rust
 use husky_lib_core::ranges::{ranges, Ranges};
 use husky_lib::acquire;

 let rs = acquire(Ranges::Capital);
 assert_eq!('A', rs[0].code() as char);
 assert_eq!('Z', rs[25].code() as char);
 ```

 ```rust
 use husky_lib_core::ranges::LETTERS;
 use husky_lib::codes;

 let cs = codes(&LETTERS);
 assert_eq!(52, cs.len());
 assert_eq!('A', cs[0].code() as char);
 assert_eq!('z', cs[51].code() as char);
 ```
