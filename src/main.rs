extern crate getopts;
extern crate regex;

use std::env;
use getopts::Options;
use std::path::{Path, PathBuf};

fn find(rgx: String, dirs: Vec<String>) {
    let mut d: Vec<PathBuf> = dirs.iter().
        map(|x| PathBuf::from(x)).collect();

    println!("{} {:?}", rgx, d)
}

fn main() {
    let args: Vec<String> = env::args().collect();     

    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        println!("Help flag passed");
        return;
    }

    if matches.free.is_empty() {
        println!("You must provide a search regex.");
        return
    }

    let rgx = matches.free[0].clone();
    let dirs = if matches.free.len() > 1 {
        let mut d = matches.free.clone();
        d.remove(0);
        d
    } else {
        vec![".".to_string()]
    };

    find(rgx, dirs)
}
