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
        
        No parameter is same as --help.

";

fn main() {
    let mut so = stdout();
    writeln!(so, "{}", INTRO);
    _ = so.flush();

    let args = std::env::args();

    if args.len() == 1 {
        write!(so, "{}", HELP);
        _ = so.flush();

        return;
    }
}
