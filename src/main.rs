extern crate getopts;
extern crate glob;
extern crate rfmod;
extern crate rand;

use getopts::Options;
use std::env;
use glob::glob;
use std::result::Result;
use std::time::Duration;
use std::thread::sleep;
use rand::{thread_rng, sample};

fn play_to_the_end(sound: rfmod::Sound, len: usize) -> rfmod::Result {
    let length = match sound.get_length(rfmod::TIMEUNIT_MS) {
        Ok(l) => l,
        Err(e) => panic!("sound.get_length error: {:?}", e)
    };
    let name = match sound.get_name(len) {
        Ok(n) => n,
        Err(e) => panic!("sound.get_name error: {:?}", e)
    };
    let mut old_position = 100usize;

    match sound.play() {
        Ok(chan) => {
            loop {
                match chan.is_playing() {
                    Ok(b) => {
                        if b == true {
                            let position = match chan.get_position(rfmod::TIMEUNIT_MS) {
                                Ok(p) => p,
                                Err(e) => {
                                    panic!("channel.get_position failed: {:?}", e)
                                }
                            };

                            if position != old_position {
                                old_position = position;
                                print!("\r{} : {:02}:{:02} / {:02}:{:02}", name, position / 1000 / 60, position / 1000 % 60,
                                    length / 1000 / 60, length / 1000 % 60);
                            }
                            sleep(Duration::new(30, 0))
                        } else {
                            break;
                        }
                    },
                    Err(e) => return e,
                }
            }
            rfmod::Result::Ok
        }
        Err(err) => err,
    }
}

fn play_music(input: &str) {
    let inpath = format!("{}/*.mp3", input);
    let files = glob(&inpath).unwrap().filter_map(Result::ok);
    let mut rng = thread_rng();
    let ref song = sample(&mut rng, files, 1)[0];
    println!("Now playing {:?}", song);
    let fmod = match rfmod::Sys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Sys::new() : {:?}", e);
        }
    };

    match fmod.init() {
        rfmod::Result::Ok => {}
        e => {
            panic!("Sys::init() failed : {:?}", e);
        }
    };

    let sound = match fmod.create_sound(song.as_os_str().to_str().unwrap(), None, None) {
        Ok(s) => s,
        Err(err) => {
            panic!("Sys::create_sound() failed : {:?}", err);
        },
    };

    match sound.play_to_the_end() {
        rfmod::Result::Ok => {
            println!("Ok !");
        },
        err => {
            panic!("Sys::play_to_the_end() : {:?}", err);
        }
    };
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
    play_music(&input);
}

