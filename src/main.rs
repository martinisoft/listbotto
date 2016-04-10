extern crate getopts;
extern crate glob;

use getopts::Options;
use std::env;
use glob::glob;
use std::result::Result;

fn mount_dir(input: &str) {
    let inpath = format!("{}/*.mp3", input);
    for path in glob(&inpath).unwrap().filter_map(Result::ok) {
        println!("{}", path.display());
    }
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: listbotto DIR [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(opts);
        return;
    };
    mount_dir(&input);
}

