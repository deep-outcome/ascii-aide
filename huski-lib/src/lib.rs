//! American Standard Code for Information Interchange table aide library.

use huski_auxies::len;
use core::ops::RangeInclusive;

pub use huski_lib_core::ranges::*;
pub use huski_lib_core::table::TABLE;

/// Similar to `fn codes()` but it accepts `&[Ranges]` as input
/// and returns merged open result.
///
/// ```
/// use huski_lib_core::ranges::Ranges;
/// use huski_lib::acquire;
///
/// let rs = acquire(&[Ranges::Capital,Ranges::Small]);
/// assert_eq!('A', rs[0].code() as char);
/// assert_eq!('z', rs[51].code() as char);
/// ```
pub fn acquire(rs: &[Ranges]) -> Vec<Code> {
    let mut len = 0;
    for r in rs {
        len += len!(ranges(r.clone()));
    }

    let mut merged = Vec::new();
    merged.reserve_exact(len);
    for r in rs {
        merged.extend(to_codes(r.clone()).into_iter())
    }
    merged
}

/// Similar to `fn codes()` but it accepts `&[Ranges]` as input
/// and returns open result in apart.
///
/// ```
/// use huski_lib_core::ranges::Ranges;
/// use huski_lib::acquire_apart;
///
/// let rs = acquire_apart(&[Ranges::Capital,Ranges::Small]);
/// assert_eq!('A', rs[0][0].code() as char);
/// assert_eq!('z', rs[1][25].code() as char);
/// ```
pub fn acquire_apart(rs: &[Ranges]) -> Vec<Vec<Code>> {
    let mut many = Vec::new();
    many.reserve_exact(rs.len());

    for r in rs {
        let codes = to_codes(r.clone());
        many.push(codes);
    }

    many
}

fn to_codes(r: Ranges) -> Vec<Code> {
    codes(ranges(r))
}

/// Provides information about ASCII code
#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    code: u8,
    human: &'static str,
    desc: &'static str,
}

/// Value acquisition.
impl Code {
    /// Decimal code value.
    pub const fn code(&self) -> u8 {
        self.code
    }

    /// Human representation.
    pub const fn human(&self) -> &'static str {
        self.human
    }

    /// Code description.
    pub const fn desc(&self) -> &'static str {
        self.desc
    }
}

/// Provides `Code`s for ranges specified.
///
/// Input values must fit into range 0-127, otherwise
/// function will panic.
///
/// Duplicities and input order are preserved.
/// ```
/// use huski_lib_core::ranges::LETTERS;
/// use huski_lib::codes;
///
/// let cs = codes(&LETTERS);
/// assert_eq!(52, cs.len());
/// assert_eq!('A', cs[0].code() as char);
/// assert_eq!('z', cs[51].code() as char);
/// ```
pub fn codes(rs: &[RangeInclusive<usize>]) -> Vec<Code> {
    let mut codes = Vec::new();
    codes.reserve_exact(len!(rs));

    let sc = codes.spare_capacity_mut();

    let mut wrix = 0;
    for r in rs {
        for i in r.clone() {
            let info = TABLE[i];
            let code = Code {
                code: i as u8,
                human: info.0,
                desc: info.1,
            };

            sc[wrix].write(code);
            wrix += 1;
        }
    }

    unsafe {
        codes.set_len(wrix);
    }

    codes
}

#[cfg(test)]
mod tests_of_units {
    use huski_lib_core::ranges::{Ranges, PRINTABLE};

    use crate::{Code, to_codes as to_codes_fn, codes};

    mod acquire {

        use huski_lib_core::ranges::{LETTERS, SYMBOLS, Ranges};
        use crate::{Code, acquire, codes};

        #[test]
        fn basic_test() {
            let proof = codes(&LETTERS);
            let test = acquire(&[Ranges::Letters]);

            assert_eq!(proof, test);
        }

        #[test]
        fn merging_test() {
            let l = codes(&LETTERS);
            let s = codes(&SYMBOLS);
            let test = acquire(&[Ranges::Letters, Ranges::Symbols]);

            let proof = l.into_iter().chain(s.into_iter()).collect::<Vec<Code>>();
            assert_eq!(proof, test);
        }
    }

    mod acquire_apart {

        use huski_lib_core::ranges::{LETTERS, SYMBOLS, Ranges};
        use crate::{acquire_apart, codes};

        #[test]
        fn basic_test() {
            let proof = vec![codes(&LETTERS)];
            let test = acquire_apart(&[Ranges::Letters]);

            assert_eq!(proof, test);
        }

        #[test]
        fn merging_test() {
            let l = codes(&LETTERS);
            let s = codes(&SYMBOLS);
            let test = acquire_apart(&[Ranges::Letters, Ranges::Symbols]);

            let proof = vec![l, s];
            assert_eq!(proof, test);
        }
    }

    #[test]
    fn to_codes() {
        let r = Ranges::Printable;
        assert_eq!(codes(&PRINTABLE), to_codes_fn(r.clone()));
    }

    #[test]
    fn code() {
        let code = 99;
        let human = "human";
        let desc = "desc";

        let test = Code { code, human, desc };

        assert_eq!(code, test.code);
        assert_eq!(human, test.human);
        assert_eq!(desc, test.desc);
    }

    mod codes {
        use crate::codes as codes_fn;
        use huski_lib_core::table::TABLE;

        #[test]
        fn basic_test() {
            let codes = [(0..=2), (125..=127)];
            let test = codes_fn(&codes);

            assert_eq!(6, test.len());
            assert!(6 <= test.capacity());

            let mut reix = 0;
            for r in codes {
                for i in r {
                    let info = TABLE[i];
                    let t = &test[reix];

                    reix += 1;

                    assert_eq!(i as u8, t.code);
                    assert_eq!(info.0, t.human);
                    assert_eq!(info.1, t.desc);
                }
            }
        }

        #[test]
        #[should_panic(expected = "index out of bounds: the len is 128 but the index is 128")]
        fn index_128() {
            let r128 = [128..=128];
            _ = codes_fn(r128.as_slice())
        }

        #[test]
        fn preservation() {
            let codes = [127..=127, 127..=127, 0..=0];

            let len = codes.len();
            let test = codes_fn(codes.as_slice());

            assert_eq!(len, test.len());

            for i in 0..len {
                let code = *codes[i].start();
                let info = TABLE[code];

                let t = &test[i];

                assert_eq!(code as u8, t.code);
                assert_eq!(info.0, t.human);
                assert_eq!(info.1, t.desc);
            }
        }
    }
}
