use std::io::stdout;
use std::io::Write;

const INTRO: &str = "

        @****************************************************************************************************@
        :     American Standard Code for Information Interchange table aide is faithful as polar dog is.     :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@
";

const HELP: &str = "
        SUBSETS
        ----------------------------
        -p       | printable
        -c       | control
        -lc      | capital letters
        -ls      | small letters
        -l       | all letters
        -d       | digits
        -s       | symbols
        
        SET
        ----------------------------
        -t       | whole table

        GENERAL
        ----------------------------
        --help   | this help
        -nt:base | number type, defaults to nt:10 = decimal, supports: binary, octal, decimal, hexadecimal
        
        No parameter is same as --help. First non-optional parameter is considered function match.
        
        EXTRA
        ----------------------------
        -v       | version info
        -r       | references
        

";

const REFERENCE: &str = "
        https://www.aivosto.com/articles/charsets-7bit.html
        https://en.wikipedia.org/wiki/ASCII
        https://www.ascii-code.com/
";

const VERSION: &str = "
        version    | 1.0.0
        repository | https://github.com/bravequickcleverfibreyarn/ascii-aide
        author     | software9119
";

#[repr(u8)]
#[derive(Clone)]
enum Base {    
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16
}

const BASE_VARIANTS: [Base; 4] = [
    Base::Binary,
    Base::Octal,
    Base::Decimal,
    Base::Hexadecimal
];

fn main() {
    let mut so = stdout();
    writeln!(so, "{}", INTRO);
    _ = so.flush();

    let args = std::env::args();
    let args = args.collect::<Vec<String>>();

    if args.len() == 1 {
        write!(so, "{}", HELP);
        _ = so.flush();

        return;
    }
    
    let mut base = Base::Decimal;
    for a in args.iter() {
        if let Some(b) = aq_base(a) {
            base = b;
            break;
        }
    }
}

fn aq_base(s: &str) -> Option<Base> {
    const NT: &str = "-nt:";
    if s.starts_with(NT) {
        if let Ok(b) = s[NT.len()..].parse::<u8>() {
            for v in BASE_VARIANTS {
                if v.clone() as u8 == b {
                    return Some(v);
                }
            }
        }
    }
    
    None
}
