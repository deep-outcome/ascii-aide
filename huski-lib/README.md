# huski-lib
Facility for working with ASCII table and its views into it. Lib component of [huski](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/huski).

See also [huski-lib-core](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/huski-lib-core).


 ```rust
use huski_lib_core::ranges::Ranges;
use huski_lib::acquire;

let rs = acquire(&[Ranges::Capital,Ranges::Small]);
assert_eq!('A', rs[0].code() as char);
assert_eq!('z', rs[51].code() as char);
 ```

 ```rust
use huski_lib_core::ranges::Ranges;
use huski_lib::acquire_apart;

let rs = acquire_apart(&[Ranges::Capital,Ranges::Small]);
assert_eq!('A', rs[0][0].code() as char);
assert_eq!('z', rs[1][25].code() as char);
 ```
 
 ```rust
 use huski_lib_core::ranges::LETTERS;
 use huski_lib::codes;

 let cs = codes(&LETTERS);
 assert_eq!(52, cs.len());
 assert_eq!('A', cs[0].code() as char);
 assert_eq!('z', cs[51].code() as char);
 ```
