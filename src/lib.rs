///! American Standard Code for Information Interchange table aide.

mod ranges {
    use core::ops::RangeInclusive;

    pub const PRINTABLE: [RangeInclusive<usize>; 1] = [(32..=126)];
    pub const CONTROL: [RangeInclusive<usize>; 2] = [(0..=31), (127..=127)];
    pub const CAPITAL: [RangeInclusive<usize>; 1] = [(65..=90)];
    pub const SMALL: [RangeInclusive<usize>; 1] = [(97..=122)];
    pub const LETTERS: [RangeInclusive<usize>; 2] = [(65..=90), (97..=122)];
    pub const DIGITS: [RangeInclusive<usize>; 1] = [(48..=57)];
    pub const SYMBOLS: [RangeInclusive<usize>; 4] = [(32..=47), (58..=64), (91..=96), (123..=126)];

    #[cfg(test)]
    mod tests_of_units {

        trait Collector<Idx>
        where
            Self: Iterator<Item = Idx> + Sized,
        {
            fn amass(self) -> Vec<Idx> {
                self.collect()
            }
        }

        impl Collector<usize> for RangeInclusive<usize> {}
        impl Collector<usize> for Chain<RangeInclusive<usize>, RangeInclusive<usize>> {}

        use super::*;
        use core::iter::Chain;

        #[test]
        fn printable() {
            let start = 0x20; // 32
            let end = 0x7e; // 126

            assert_eq!(1, PRINTABLE.len());
            let printable = PRINTABLE[0].clone();

            let proof = start..=end;

            assert_eq!(proof.amass(), printable.amass());
        }

        #[test]
        fn control() {
            let start = 0x0; // 0
            let end = 0x1f; // 31
            let extra = 0x7f; // 127

            assert_eq!(2, CONTROL.len());
            let control = CONTROL[0].clone().chain(CONTROL[1].clone());

            let mut proof = (start..=end).amass();
            proof.push(extra);

            assert_eq!(proof, control.amass());
        }

        #[test]
        fn capital() {
            let start = 'A' as usize; // 65
            let end = 'Z' as usize; // 90

            assert_eq!(1, CAPITAL.len());
            let capital = CAPITAL[0].clone();

            let proof = start..=end;

            assert_eq!(proof.amass(), capital.amass());
        }

        #[test]
        fn small() {
            let start = 'a' as usize; // 97
            let end = 'z' as usize; // 122

            assert_eq!(1, SMALL.len());
            let small = SMALL[0].clone();

            let proof = start..=end;

            assert_eq!(proof.amass(), small.amass());
        }

        #[test]
        fn letters() {
            let start_c = 'A' as usize;
            let end_c = 'Z' as usize;

            let start_s = 'a' as usize;
            let end_s = 'z' as usize;

            assert_eq!(2, LETTERS.len());
            let letters = LETTERS[0].clone().chain(LETTERS[1].clone());

            let proof = (start_c..=end_c).chain(start_s..=end_s);

            assert_eq!(proof.amass(), letters.amass());
        }

        #[test]
        fn digits() {
            let start = '0' as usize; // 48
            let end = '9' as usize; // 57

            assert_eq!(1, DIGITS.len());
            let digits = DIGITS[0].clone();

            let proof = start..=end;

            assert_eq!(proof.amass(), digits.amass());
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
            let mut symbols = SYMBOLS[0].clone().chain(SYMBOLS[1].clone()).amass();
            symbols.extend_from_slice(&(SYMBOLS[2].clone()).chain(SYMBOLS[3].clone()).amass());

            let mut proof = (start_1..=end_1).chain(start_2..=end_2).amass();
            proof.extend_from_slice(&(start_3..=end_3).chain(start_4..=end_4).amass());

            assert_eq!(proof, symbols);
        }
    }
}

mod table {
    pub const TABLE: [(&str, &str); 128] = [
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
        ("x", ""),
        ("y", ""),
        ("z", ""),
        ("{", "Left brace"),
        ("|", "Verical line"),
        ("}", "Right brace"),
        ("~", "Tilde"),
        ("DEL", "Delete"),
    ];
}
