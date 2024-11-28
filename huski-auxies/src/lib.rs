//! Auxiliaries for huski implementation.

/// Collects and concats `RangeInclusive<usize>`s of
/// literal listing into `Vec<usize>`.
///
/// ```
/// use huski_auxies::ccr1;
/// let rs = ccr1!(0..=2,0..=3);
/// assert_eq!(0, rs[3]);
/// ```
#[macro_export]
macro_rules! ccr1 {
    ( $($r:expr),*) => {
        {
            let mut rs = Vec::<usize>::new();

            let mut cap = 0;
            $(
                cap += $r.end() - $r.start() + 1;
            )*

            rs.reserve_exact(cap);

            $(
                let sc = rs.spare_capacity_mut();
                let mut wix = 0;
                for i in $r.clone() {
                    sc[wix].write(i);
                    wix +=1;
                }

                unsafe {
                    rs.set_len(rs.len() + wix);
                }
            )*

            rs
        }
    }
}

/// Collects and concats `RangeInclusive<usize>`s from
/// withing for-loop iteratable into `Vec<usize>`.
///
/// ```
/// use huski_auxies::{len, ccr1, ccr2};
/// let rs = ccr2!(&[0..=2,0..=3]);
/// assert_eq!(0, rs[3]);
/// ```
#[macro_export]
macro_rules! ccr2 {
    ($i:expr) => {{
        let mut rs = Vec::<usize>::new();

        let cap = len!($i);
        rs.reserve_exact(cap);

        for r in $i {
            rs.extend_from_slice(&ccr1!(r));
        }

        rs
    }};
}

#[macro_export]
/// Computes cumulative length of `RangeInclusive`s from within for-loop iteratable.
///
/// ```
/// use huski_auxies::len;
/// let len = len!(&[0..=2,0..=3]);
/// assert_eq!(7, len);
/// ```
macro_rules! len {
    ($i:expr) => {{
        let mut len = 0;
        for r in $i {
            len += r.end() - r.start() + 1;
        }

        len
    }};
}

#[cfg(test)]
mod tests_of_units {

    #[test]
    fn ccr1() {
        let r_1 = 0..=2;
        let r_2 = 0..=3;

        let rs = ccr1!(&r_1, &r_2);

        assert_eq!(7, rs.len());
        assert!(7 <= rs.capacity());
        assert_eq!(2, rs[2]);
        assert_eq!(3, rs[6]);
    }

    #[test]
    fn ccr2() {
        let r_1 = 0..=2;
        let r_2 = 0..=3;
        let rs = [r_1, r_2];

        let test = ccr2!(&rs);

        assert_eq!(7, test.len());
        assert!(7 <= test.capacity());
        assert_eq!(2, test[2]);
        assert_eq!(3, test[6]);
    }

    #[test]
    fn len() {
        let r_1 = 0..=2;
        let r_2 = 0..=3;
        let rs = [r_1, r_2];

        let len = len!(&rs);
        assert_eq!(7, len);
    }
}
