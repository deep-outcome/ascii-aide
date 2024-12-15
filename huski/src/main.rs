use std::io::{stdout, Write};
use huski_lib::{acquire, acquire_apart, Ranges, Code};

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
        -tt:type | table type, defaults to tt:c = classic order, supports: s — special, c — classic 
                 | if -tt:s prints subset ordered-table in order: lc,ls,d,s,c, works only with -t
        
        No parameter is same as --help. First known non-optional parameter is considered function match.
        Similarly, first valid optional parameter is considered match.
        
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
        version    | 1.1.0
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
    (PRINTABLE_SUBSET_NAME, Ranges::Printable),
    ("c", Ranges::Control),
    ("lc", Ranges::Capital),
    ("ls", Ranges::Small),
    (LETTERS_SUBSET_NAME, Ranges::Letters),
    ("d", Ranges::Digits),
    ("s", Ranges::Symbols),
    (TABLE_SET_NAME, Ranges::Table),
];

const PRINTABLE_SUBSET_NAME: &str = "p";
const LETTERS_SUBSET_NAME: &str = "l";
const TABLE_SET_NAME: &str = "t";

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

    let mut table_output = false;
    let mut ranges = vec![];
    'param: for a in args.iter() {
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
            | pmtr => {
                if !pmtr.starts_with("-") {
                    continue;
                }

                let name = &pmtr[1..];
                if !ranges_map.contains_key(name) {
                    continue;
                }

                if name == TABLE_SET_NAME {
                    table_output = true;

                    for a2 in args.iter() {
                        const TABLE_TYPE_FLAG: &str = "-tt:";
                        if a2.starts_with(TABLE_TYPE_FLAG) {
                            let tt = &a2[TABLE_TYPE_FLAG.len()..];

                            match tt {
                                | "s" => {
                                    ranges.push(Ranges::Capital);
                                    ranges.push(Ranges::Small);
                                    ranges.push(Ranges::Digits);
                                    ranges.push(Ranges::Symbols);
                                    ranges.push(Ranges::Control);

                                    break 'param;
                                },
                                | "c" => break,
                                | _ => continue,
                            }
                        }
                    }
                }

                let r = ranges_map.get(name).unwrap();
                ranges.push(r.clone());
                break;
            },
        }
    }

    if ranges.len() == 0 {
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

    let ranges = ranges.as_slice();
    let mut output = String::with_capacity(3000);

    if table_output {
        let apart = acquire_apart(ranges);

        let special = apart.len() > 1;
        let codes = if special {
            let mut codes = Vec::new();
            codes.reserve_exact(5 * 33);
            for item in apart.iter() {
                for i in 0..33 {
                    codes.push(item.get(i))
                }
            }

            codes
        } else {
            apart[0].iter().map(|x| Some(x)).collect()
        };

        set(codes.as_slice(), &mut output, base, special);
    } else {
        let codes = acquire(ranges);
        subset(codes.as_slice(), &mut output, base);
    };

    write(output.as_str());
}

fn set(codes: &[Option<&Code>], o: &mut String, b: Base, special: bool) {
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

    if special {
        o.push_str(HEADER);
    }

    o.push('|');
    o.push('\n');
    o.push_str(INDENT);
    o.push_str("---------------------------------------------------------------------------------");

    if special {
        o.push_str("--------------------");
    }

    let (cols, last_col_ix, rows): (&[usize], usize, usize) = if special {
        const COLS: [usize; 5] = [0, 33, 66, 99, 132];
        (&COLS, 132, 33)
    } else {
        const COLS: [usize; 4] = [0, 32, 64, 96];
        (&COLS, 96, 32)
    };

    for row_ix in 0..rows {
        o.push('\n');
        o.push_str(INDENT);

        for &col_ix in cols.iter() {
            let ix = col_ix + row_ix;

            if let Some(c) = &codes[ix] {
                let numeric = f(c.code());
                let human = format!("{:<7}", c.human());
                o.push_str(numeric.as_str());
                o.push_str(human.as_str());
            } else {
                o.push_str("| ---       ");
                o.push_str("| -     ");
            };

            if col_ix == last_col_ix {
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

fn subset(codes: &[Code], o: &mut String, b: Base) {
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

    for c in codes.iter() {
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
    const NUMBER_TYPE_FLAG: &str = "-nt:";
    if s.starts_with(NUMBER_TYPE_FLAG) {
        if let Ok(b) = s[NUMBER_TYPE_FLAG.len()..].parse::<u8>() {
            for v in BASE_VARIANTS.iter().cloned() {
                if v.clone() as u8 == b {
                    return Some(v);
                }
            }
        }
    }

    None
}
