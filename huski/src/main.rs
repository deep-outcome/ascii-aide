use std::io::{stdout, Write};
use huski_lib::{acquire, Ranges, Code};

const INTRO: &str = "\n\n
        @***************************************************************************************************@
        :    American Standard Code for Information Interchange table aide is faithful as arctic dog is.    :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@";

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
        
        No parameter is same as --help. First known non-optional parameter is considered function match.
        
        EXTRA
        ----------------------------
        -v       | version info
        -r       | references";

const REFERENCE: &str = "
        REFERENCE
        ---------
        https://www.aivosto.com/articles/charsets-7bit.html
        https://en.wikipedia.org/wiki/ASCII
        https://www.ascii-code.com";

const VERSION: &str = "
        version    | 1.0.4
        repository | https://github.com/bravequickcleverfibreyarn/ascii-aide
        author     | software9119.technology";

const ERR_NO_PARAM: &str = "\n
        Err:: no valid function parameter found";

const INDENT: &str = "        ";

#[derive(Clone)]
enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}

static BASE_VARIANTS: [Base; 4] = [Base::Binary, Base::Octal, Base::Decimal, Base::Hexadecimal];

static RANGES_MAP: [(&str, Ranges); 8] = [
    ("-p", Ranges::Printable),
    ("-c", Ranges::Control),
    ("-lc", Ranges::Capital),
    ("-ls", Ranges::Small),
    ("-l", Ranges::Letters),
    ("-d", Ranges::Digits),
    ("-s", Ranges::Symbols),
    ("-t", Ranges::Table),
];

fn main() {
    write(INTRO);

    let args = std::env::args();
    let args = args.collect::<Vec<String>>();

    if args.len() == 1 {
        write(HELP);
        return;
    }

    use std::collections::HashMap;
    let ranges_map = RANGES_MAP.iter().cloned();
    let ranges_map: HashMap<&str, Ranges> = HashMap::from_iter(ranges_map);

    let mut ranges = None;
    for a in args.iter() {
        match a.as_str() {
            | "-v" => {
                write(VERSION);
                return;
            },
            | "-r" => {
                write(REFERENCE);
                return;
            },
            | "--help" => {
                write(HELP);
                return;
            },
            | x if ranges_map.contains_key(x) => {
                let r = ranges_map.get(x).unwrap();
                ranges = Some(r.clone());
                break;
            },
            | _ => {},
        }
    }

    if ranges.is_none() {
        write(ERR_NO_PARAM);
        return;
    }

    let mut base = Base::Decimal;
    for a in args {
        if let Some(b) = aq_base(a.as_str()) {
            base = b;
            break;
        }
    }

    let ranges = ranges.unwrap();
    let codes = acquire(ranges.clone());

    let mut output = String::with_capacity(3000);

    if ranges == Ranges::Table {
        set(codes, &mut output, base);
    } else {
        subset(codes, &mut output, base);
    };

    write(output.as_str());
}

fn set(cs: Box<[Code]>, o: &mut String, b: Base) {
    let f = match b {
        | Base::Binary => b_set,
        | Base::Octal => o_set,
        | Base::Decimal => d_set,
        | Base::Hexadecimal => h_set,
    };

    const HEADER: &str = "|  NUMERIC  | HUMAN ";
    o.push('\n');
    o.push_str(INDENT);
    o.push_str(HEADER);
    o.push_str(HEADER);
    o.push_str(HEADER);
    o.push_str(HEADER);
    o.push('|');
    o.push('\n');
    o.push_str(INDENT);
    o.push_str("---------------------------------------------------------------------------------");

    let cols = [0, 32, 64, 96];
    for row in 0..32 {
        o.push('\n');
        o.push_str(INDENT);

        for &col in cols.iter() {
            let code = col + row;

            let c = &cs[code];
            let numeric = f(c.code());
            o.push_str(numeric.as_str());
            let human = format!("{:<7}", c.human());
            o.push_str(human.as_str());

            if col == 96 {
                o.push('|');
            }
        }
    }

    fn b_set(c: u8) -> String {
        format!("| 0b{:>07b} |", c)
    }

    fn o_set(c: u8) -> String {
        let zero_pad = format!("| 0o{:>03o}", c);
        format!("{:<11} |", zero_pad)
    }

    fn d_set(c: u8) -> String {
        format!("| {:<9} |", c)
    }

    fn h_set(c: u8) -> String {
        let zero_pad = format!("| 0x{:>02x}", c);
        format!("{:<11} |", zero_pad)
    }
}

fn subset(cs: Box<[Code]>, o: &mut String, b: Base) {
    let f = match b {
        | Base::Binary => b_subset,
        | Base::Octal => o_subset,
        | Base::Decimal => d_subset,
        | Base::Hexadecimal => h_subset,
    };

    const HEADER: &str = "  NUMERIC  | HUMAN | DESCRIPTION";
    o.push('\n');
    o.push_str(INDENT);
    o.push_str(HEADER);
    o.push('\n');
    o.push_str(INDENT);
    o.push_str("-------------------------------------------------");

    for c in cs.iter() {
        o.push('\n');
        o.push_str(INDENT);

        let numeric = f(c.code());
        o.push_str(numeric.as_str());
        let human = format!("{:^7}", c.human());
        o.push_str(human.as_str());
        o.push_str("| ");
        o.push_str(c.desc());
    }

    fn b_subset(c: u8) -> String {
        format!(" 0b{:>07b} |", c)
    }

    fn o_subset(c: u8) -> String {
        let zero_pad = format!(" 0o{:>03o}", c);
        format!("{:^10} |", zero_pad)
    }

    fn d_subset(c: u8) -> String {
        format!(" {:^9} |", c)
    }

    fn h_subset(c: u8) -> String {
        let zero_pad = format!("0x{:>02x}", c);
        format!("{:^10} |", zero_pad)
    }
}

fn write(s: &str) {
    let mut so = stdout();
    if let Ok(_) = write!(so, "{}\n\n", s) {
        if let Ok(_) = so.flush() {
            return;
        }
    }

    panic!("Problem writing output");
}

fn aq_base(s: &str) -> Option<Base> {
    const NT: &str = "-nt:";
    if s.starts_with(NT) {
        if let Ok(b) = s[NT.len()..].parse::<u8>() {
            for v in BASE_VARIANTS.iter().cloned() {
                if v.clone() as u8 == b {
                    return Some(v);
                }
            }
        }
    }

    None
}
