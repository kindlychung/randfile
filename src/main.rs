#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;
extern crate rand;

use rand::random;
use std::path;
use std::env;

fn rand_string() -> String {
    (0..15).map(|_| (97u8 + (random::<f32>() * 25.0) as u8) as char).collect()
}


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
    let current_dir: path::PathBuf = env::current_dir().unwrap();
    let mut out_path = if &args.arg_parentdir != "" {
        let mut d = path::PathBuf::new();
        d.push(&args.arg_parentdir);
        d
    } else {
        current_dir
    };
    let filename = format!("{}{}.{}",
                           &args.flag_startwith,
                           rand_string(),
                           &args.arg_ext);
    out_path.push(&filename);
    print!("{:?}", out_path);
}
