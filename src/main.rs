extern crate regex;
extern crate clap;
extern crate walkdir;

use std::env;
use std::path::{Path, PathBuf};
use clap::{Arg, App, AppSettings};

fn find(rgx: String, dirs: Vec<PathBuf>) {}

fn main() {
    let matches = App::new("fnd, a simpler way to find files")
        .version("0.1.0")
        .author("Mathew Robinson <mrobinson@praelatus.io>")
        .about("Finds files")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("regex")
            .short("rg")
            .long("regex")
            .help("Specifies to treat SEARCH as a regex or fuzzy find (default)"))
        .arg(Arg::with_name("SEARCH")
            .required(true)
            .index(1)
            .help("The term to search for"))
        .arg(Arg::with_name("DIRS")
            .multiple(true)
            .help("Where to search for SEARCH")
            .default_value("."))
        .get_matches();

    let search = matches.value_of("SEARCH").unwrap();
    let dirs: Vec<&str> = matches.values_of("DIRS").unwrap().collect();

    println!("search: {}", search);
    println!("dirs: {:?}", dirs);

    let mut d: Vec<PathBuf> = dirs.iter()
        .map(|x| {
            if *x == "." {
                return env::current_dir().unwrap().to_path_buf();
            }

            PathBuf::from(x)
        })
        .collect();

    println!("{:?}", d);
}
