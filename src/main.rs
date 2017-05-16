extern crate regex;
extern crate clap;
extern crate walkdir;
extern crate ansi_term;

use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

use clap::{Arg, App, AppSettings};
use regex::Regex;
use walkdir::WalkDir;
use ansi_term::Colour::Blue;

use std::env;
use std::path::PathBuf;

fn find(rgx: Regex, dirs: Vec<PathBuf>, paint: bool) {
    let painter;
    let replacement = if paint {
        painter = Blue.paint("$search");
        format!("{}", painter)
    } else {
        "$search".to_string()
    };

    for d in dirs.iter() {
        let wkd = WalkDir::new(d);

        for f in wkd.into_iter() {
            let entry;
            match f {
                Ok(e) => entry = e,
                Err(err) => {
                    println_stderr!("{}", err);
                    continue;
                }
            }

            // paths are ascii so should always be valid unicode
            // the edge case is super rare so just ignore it
            let s = &entry.path().to_string_lossy();

            if rgx.is_match(s) {
                let display = rgx.replace_all(s, replacement.as_str());
                println!("{}", display);
            }
        }
    }
}

fn main() {
    let matches = App::new("fnd, a simpler way to find files")
        .version("0.2.0")
        .author("Mathew Robinson <mrobinson@praelatus.io>")
        .about("Finds files")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("color")
             .long("no_color")
             .short("nc")
             .help("When specified will not highlight matches with ansi \
                    term color codes this is useful when piping output"))
        .arg(Arg::with_name("REGEX")
             .required(true)
             .index(1)
             .help("The REGEX to search for, defaults to fuzzy finding"))
        .arg(Arg::with_name("DIRS")
            .multiple(true)
            .help("Where to search for SEARCH")
            .default_value("."))
        .get_matches();

    let search = matches.value_of("REGEX").unwrap();
    let dirs: Vec<&str> = matches.values_of("DIRS").unwrap().collect();

    let d: Vec<PathBuf> = dirs.iter()
        .map(|x| {
            if *x == "." {
                return env::current_dir().unwrap().to_path_buf();
            }

            PathBuf::from(x)
        })
        .collect();

    let re = format!("(?P<search>{})", search);
    let rgx = Regex::new(&re).unwrap();

    find(rgx, d, !matches.is_present("color"));
}
