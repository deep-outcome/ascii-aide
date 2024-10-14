//! American Standard Code for Information Interchange table aide library.

use husky_auxies::len;
use core::ops::RangeInclusive;

pub use husky_lib_core::ranges::*;
pub use husky_lib_core::table::TABLE;

/// Same as `fn codes()` but it accepts `Ranges` as input
/// and returns _boxed_ result.
pub fn aquire(r: Ranges) -> Box<[Code]> {
    let rs = ranges(r);
    let cs = codes(&rs);
    cs.into_boxed_slice()
}

/// Provides information about ASCII code.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Code {
    code: usize,
    name: &'static str,
    desc: &'static str,
}

/// Value acquisition.
impl Code {
    /// Decimal code value.
    pub fn code(&self) -> usize {
        self.code
    }

    /// Code name.
    pub fn name(&self) -> &'static str {
        self.name
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
pub fn codes(rs: &[RangeInclusive<usize>]) -> Vec<Code> {
    let mut codes = Vec::new();
    codes.reserve_exact(len!(rs));

    let sc = codes.spare_capacity_mut();

    let mut wrix = 0;
    for r in rs {
        for i in r.clone() {
            let info = TABLE[i];
            let code = Code {
                code: i,
                name: info.0,
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
    use crate::{Code, aquire as aquire_fn, codes as codes_fn};

    #[test]
    fn aquire() {
        let cs = codes_fn(&LETTERS);
        let test = aquire_fn(Ranges::Letters);

        assert_eq!(cs.as_slice(), &*test);
    }

    #[test]
    fn code() {
        let code = 99;
        let name = "name";
        let desc = "desc";

        let test = Code { code, name, desc };

        assert_eq!(code, test.code);
        assert_eq!(name, test.name);
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

                    assert_eq!(i, t.code);
                    assert_eq!(info.0, t.name);
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

                assert_eq!(code, t.code);
                assert_eq!(info.0, t.name);
                assert_eq!(info.1, t.desc);
            }
        }
    }
}
