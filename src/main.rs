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
                let display = if paint {
                    rgx.replace_all(s, replacement.as_str()).to_owned()
                } else {
                    s.to_owned()
                };

                println!("{}", display);
            }
        }
    }
}

fn main() {
    let matches = App::new("fnd")
        .version("0.3.0")
        .author("Mathew Robinson <mrobinson@praelatus.io>")
        .about("
Find files by regex.

Copyright (C) 2016 Mathew Robinson <mrobinson@praelatus.io>

This program is free software: you can redistribute it and/or modify
it under the terms of the Apache Version 2.0 License

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

You should have recieved a copy of the license with this software if
not you can view it here: https://www.apache.org/licenses/LICENSE-2.0")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("color")
             .long("color")
             .short("c")
             .help("When specified will highlight matches with ansi \
                    term color codes. Note that for large regexes or \
                    regexes which match a large portion of text this \
                    can negatively affect performance."))
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

    find(rgx, d, matches.is_present("color"));
}
