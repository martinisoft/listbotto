extern crate getopts;
extern crate iron;
extern crate glob;
extern crate rfmod;
extern crate rand;
#[macro_use] extern crate mime;

use getopts::Options;
use std::env;
// use iron::prelude::*;
// use iron::status;

mod player;

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
    player::play_music(&input);
}

