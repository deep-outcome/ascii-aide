//! American Standard Code for Information Interchange table aide library.

use husky_auxies::len;
use core::ops::RangeInclusive;

pub use husky_lib_core::ranges::*;
pub use husky_lib_core::table::TABLE;

/// Same as `fn codes()` but it accepts `Ranges` as input
/// and returns _boxed_ result.
///
/// ```
/// use husky_lib_core::ranges::{ranges, Ranges};
/// use husky_lib::acquire;
///
/// let rs = acquire(Ranges::Capital);
/// assert_eq!('A', rs[0].code() as char);
/// assert_eq!('Z', rs[25].code() as char);
/// ```
pub fn acquire(r: Ranges) -> Box<[Code]> {
    let rs = ranges(r);
    let cs = codes(&rs);
    cs.into_boxed_slice()
}

/// Provides information about ASCII code.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Code {
    code: u8,
    human: &'static str,
    desc: &'static str,
}

/// Value acquisition.
impl Code {
    /// Decimal code value.
    pub fn code(&self) -> u8 {
        self.code
    }

    /// Human representation.
    pub fn human(&self) -> &'static str {
        self.human
    }

    /// Code description.
    pub fn desc(&self) -> &'static str {
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
/// use husky_lib_core::ranges::LETTERS;
/// use husky_lib::codes;
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
    use husky_lib_core::ranges::{LETTERS, Ranges};
    use crate::{Code, acquire as acquire_fn, codes as codes_fn};

    #[test]
    fn acquire() {
        let cs = codes_fn(&LETTERS);
        let test = acquire_fn(Ranges::Letters);

        assert_eq!(cs.as_slice(), &*test);
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
        use husky_lib_core::table::TABLE;

        #[test]
        fn basic_test() {
            let codes = [(0..=2), (125..=127)];
            let test = codes_fn(&codes);

            assert_eq!(6, test.len());
            assert_eq!(6, test.capacity());

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
