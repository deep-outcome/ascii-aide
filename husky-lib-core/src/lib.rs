#![no_std]
//! American Standard Code for Information Interchange table aide no-std library core.

/// Module contains ASCII code ranges information.
pub mod ranges {

    /// Provides `&[RangeInclusive<usize>]` for demand of `r`.
    /// ```
    /// use husky_lib_core::ranges::{ranges, Ranges};    
    ///
    /// let rs = ranges(Ranges::Capital);
    /// assert_eq!('A', rs[0].clone().min().unwrap() as u8 as char);
    /// assert_eq!('Z', rs[0].clone().max().unwrap() as u8 as char);
    /// ```
    pub fn ranges(r: Ranges) -> &'static [RangeInclusive<usize>] {
        match r {
            | Ranges::Printable => &PRINTABLE,
            | Ranges::Control => &CONTROL,
            | Ranges::Capital => &CAPITAL,
            | Ranges::Small => &SMALL,
            | Ranges::Letters => &LETTERS,
            | Ranges::Digits => &DIGITS,
            | Ranges::Symbols => &SYMBOLS,
            | Ranges::Table => &TABLE,
        }
    }

    /// Code ranges.
    #[derive(Clone)]
    pub enum Ranges {
        /// Printable codes
        Printable,
        /// Control codes
        Control,
        /// Capital letter codes
        Capital,
        /// Small letter codes
        Small,
        /// All letter codes
        Letters,
        /// Digit codes
        Digits,
        /// Symbols codes
        Symbols,
        /// Whole table codes
        Table,
    }

    use core::ops::RangeInclusive;

    /// Printable codes
    pub static PRINTABLE: [RangeInclusive<usize>; 1] = [(32..=126)];
    /// Control codes
    pub static CONTROL: [RangeInclusive<usize>; 2] = [(0..=31), (127..=127)];
    /// Capital letter codes
    pub static CAPITAL: [RangeInclusive<usize>; 1] = [(65..=90)];
    /// Small letter codes
    pub static SMALL: [RangeInclusive<usize>; 1] = [(97..=122)];
    /// All letter codes
    pub static LETTERS: [RangeInclusive<usize>; 2] = [(65..=90), (97..=122)];
    /// Digit codes
    pub static DIGITS: [RangeInclusive<usize>; 1] = [(48..=57)];
    /// Symbols codes
    pub static SYMBOLS: [RangeInclusive<usize>; 4] = [(32..=47), (58..=64), (91..=96), (123..=126)];
    /// Whole table codes
    pub static TABLE: [RangeInclusive<usize>; 1] = [(0..=127)];

    #[cfg(test)]
    mod tests_of_units {

        extern crate std;
        use std::vec::Vec;
        use super::{ranges as ranges_fn, *};        
        use husky_auxies::{ccr1, ccr2, len};

        #[test]
        fn ranges() {
            assert_eq!(&PRINTABLE, ranges_fn(Ranges::Printable));
            assert_eq!(&CONTROL, ranges_fn(Ranges::Control));
            assert_eq!(&CAPITAL, ranges_fn(Ranges::Capital));
            assert_eq!(&SMALL, ranges_fn(Ranges::Small));
            assert_eq!(&LETTERS, ranges_fn(Ranges::Letters));
            assert_eq!(&DIGITS, ranges_fn(Ranges::Digits));
            assert_eq!(&SYMBOLS, ranges_fn(Ranges::Symbols));
            assert_eq!(&TABLE, ranges_fn(Ranges::Table));
        }

        #[test]
        fn printable() {
            let start = 0x20; // 32
            let end = 0x7e; // 126

            assert_eq!(1, PRINTABLE.len());

            let printable = ccr2!(&PRINTABLE);
            let proof = ccr1!(start..=end);

            assert_eq!(proof, printable);
        }

        #[test]
        fn control() {
            let start = 0x0; // 0
            let end = 0x1f; // 31
            let extra = 0x7f; // 127

            assert_eq!(2, CONTROL.len());

            let control = ccr2!(&CONTROL);
            let proof = ccr1!(start..=end, extra..=extra);

            assert_eq!(proof, control);
        }

        #[test]
        fn capital() {
            let start = 'A' as usize; // 65
            let end = 'Z' as usize; // 90

            assert_eq!(1, CAPITAL.len());

            let capital = ccr2!(&CAPITAL);
            let proof = ccr1!(start..=end);

            assert_eq!(proof, capital);
        }

        #[test]
        fn small() {
            let start = 'a' as usize; // 97
            let end = 'z' as usize; // 122

            assert_eq!(1, SMALL.len());

            let small = ccr2!(&SMALL);
            let proof = ccr1!(start..=end);

            assert_eq!(proof, small);
        }

        #[test]
        fn letters() {
            let start_c = 'A' as usize;
            let end_c = 'Z' as usize;

            let start_s = 'a' as usize;
            let end_s = 'z' as usize;

            assert_eq!(2, LETTERS.len());

            let letters = ccr2!(&LETTERS);
            let proof = ccr1!(start_c..=end_c, start_s..=end_s);

            assert_eq!(proof, letters);
        }

        #[test]
        fn digits() {
            let start = '0' as usize; // 48
            let end = '9' as usize; // 57

            assert_eq!(1, DIGITS.len());
            let digits = ccr2!(&DIGITS);
            let proof = ccr1!(start..=end);

            assert_eq!(proof, digits);
        }

        #[test]
        fn symbols() {
            let start_1 = ' ' as usize; // 32
            let end_1 = '/' as usize; // 47

            let start_2 = ':' as usize; // 58
            let end_2 = '@' as usize; // 64

            let start_3 = '[' as usize; // 91
            let end_3 = '`' as usize; // 96

            let start_4 = '{' as usize; // 123
            let end_4 = '~' as usize; // 126

            assert_eq!(4, SYMBOLS.len());
            let symbols = ccr2!(&SYMBOLS);
            let proof = ccr1!(
                start_1..=end_1,
                start_2..=end_2,
                start_3..=end_3,
                start_4..=end_4
            );

            assert_eq!(proof, symbols);
        }
    }
}

/// Module contains ASCII code table information.
pub mod table {

    /// ASCII table information.
    pub static TABLE: [(&str, &str); 128] = [
        ("NUL", "Null"),
        ("SOH", "Start of heading"),
        ("STX", "Start of text"),
        ("ETX", "End of text"),
        ("EOT", "End of transmission"),
        ("ENQ", "Enquiry"),
        ("ACK", "Acknowledgement"),
        ("BEL", "Bell"),
        ("BS", "Backspace"),
        ("HT", "Horizontal tab"),
        ("LF", "Line feed"),
        ("VT", "Vertical tab"),
        ("FF", "Form feed"),
        ("CR", "Carriage return"),
        ("SO", "Shift out"),
        ("SI", "Shift in"),
        ("DLE", "Data link escape"),
        ("DC1", "Device control 1"),
        ("DC2", "Device control 2"),
        ("DC3", "Device control 3"),
        ("DC4", "Device control 4"),
        ("NAK", "Negative acknowlegment"),
        ("SYN", "Synchronous idle"),
        ("ETB", "End of transmission block"),
        ("CAN", "Cancel"),
        ("EM", "End of medium"),
        ("SUB", "Substitude"),
        ("ESC", "Escape"),
        ("FS", "File separator"),
        ("GS", "Group separator"),
        ("RS", "Record separator"),
        ("US", "Unit separator"),
        (" ", "Space"),
        ("!", "Exlamation mark"),
        ("\"", "Double quotation mark"),
        ("#", "Number sign"),
        ("$", "Dollar sign"),
        ("%", "Percent sign"),
        ("&", "Ampersand"),
        ("'", "Apostrophe"),
        ("(", "Left parenthesis"),
        (")", "Right parenthesis"),
        ("*", "Asterisk"),
        ("+", "Plus sign"),
        (",", "Comma"),
        ("-", "Hyphen/Minus sign"),
        (".", "Period"),
        ("/", "Solidus"),
        ("0", ""),
        ("1", ""),
        ("2", ""),
        ("3", ""),
        ("4", ""),
        ("5", ""),
        ("6", ""),
        ("7", ""),
        ("8", ""),
        ("9", ""),
        (":", "Colon"),
        (";", "Semicolon"),
        ("<", "Less-than sign"),
        ("=", "Equals sign"),
        (">", "Greater-than sign"),
        ("?", "Question mark"),
        ("@", "At sign"),
        ("A", ""),
        ("B", ""),
        ("C", ""),
        ("D", ""),
        ("E", ""),
        ("F", ""),
        ("G", ""),
        ("H", ""),
        ("I", ""),
        ("J", ""),
        ("K", ""),
        ("L", ""),
        ("M", ""),
        ("N", ""),
        ("O", ""),
        ("P", ""),
        ("Q", ""),
        ("R", ""),
        ("S", ""),
        ("T", ""),
        ("U", ""),
        ("V", ""),
        ("W", ""),
        ("X", ""),
        ("Y", ""),
        ("Z", ""),
        ("[", "Left bracket"),
        ("\\", "Reverse solidus"),
        ("]", "Right bracket"),
        ("^", "Circumflex accent"),
        ("_", "Underscore"),
        ("`", "Grave accent"),
        ("a", ""),
        ("b", ""),
        ("c", ""),
        ("d", ""),
        ("e", ""),
        ("f", ""),
        ("g", ""),
        ("h", ""),
        ("i", ""),
        ("j", ""),
        ("k", ""),
        ("l", ""),
        ("m", ""),
        ("n", ""),
        ("o", ""),
        ("p", ""),
        ("q", ""),
        ("r", ""),
        ("s", ""),
        ("t", ""),
        ("u", ""),
        ("v", ""),
        ("w", ""),
        ("ccr", ""),
        ("y", ""),
        ("z", ""),
        ("{", "Left brace"),
        ("|", "Verical line"),
        ("}", "Right brace"),
        ("~", "Tilde"),
        ("DEL", "Delete"),
    ];
}
