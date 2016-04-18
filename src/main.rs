#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;
extern crate randfile;

use randfile::filename::rand_file;



docopt!(Args derive Debug, "
randfile

Usage:
    randfile  [--startwith=<start>] <ext> [<parentdir>]
    randfile  (--help | -h)

Options:
    --startwith=<start>    Start the filename with the <start> string.
    --help                 Print this message.
    -h                     Print this message.

Arguments:
    ext       Extension name.
    parentdir    Parent directory of the file.

Examples:
    target/debug/randfile --help
    target/debug/randfile --startwith=tmp_ rs /tmp
    target/debug/randfile rs /tmp
    target/debug/randfile rs
    target/debug/randfile --startwith=tmp_ rs
",
flag_startwith: String,
arg_ext: String,
arg_parentdir: String
);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    if args.flag_help | args.flag_h {
        std::process::exit(0);
    }
    let ext = args.arg_ext;
    let startwith = args.flag_startwith;
    let parentdir = args.arg_parentdir;
    let rf = rand_file(&parentdir, &startwith, &ext);
    print!("{:?}", rf);
}
