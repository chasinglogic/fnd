extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate walkdir;

use std::io;
use std::io::Write;
use std::process;

use ansi_term::Colour::Blue;
use clap::{App, AppSettings, Arg};
use regex::Regex;

use walkdir::{DirEntry, WalkDir};

use std::env;
use std::path::PathBuf;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn find<T>(
    rgx: Regex,
    dirs: T,
    ignore_hidden: bool,
    replacement: &Option<String>,
) -> Option<io::Error>
where
    T: IntoIterator<Item = PathBuf>,
{
    for dir in dirs {
        let wkd: Box<Iterator<Item = walkdir::Result<DirEntry>>> = if ignore_hidden {
            Box::new(
                WalkDir::new(dir)
                    .into_iter()
                    .filter_entry(|d| !is_hidden(d)),
            )
        } else {
            Box::new(WalkDir::new(dir).into_iter())
        };

        for file in wkd {
            let entry = match file {
                Ok(e) => e,
                Err(err) => {
                    let write_error = writeln!(&mut io::stderr(), "{}", err);
                    if let Err(e) = write_error {
                        return Some(e);
                    }

                    continue;
                }
            };

            // paths are ascii or utf-8 so should always be valid unicode
            // the edge case is super rare so just ignore it
            let s = &entry.path().to_string_lossy();
            if rgx.is_match(s) {
                let write_error = writeln!(
                    &mut io::stdout(),
                    "{}",
                    match replacement {
                        Some(repl) => rgx.replace_all(s, repl.as_str()).to_owned(),
                        None => s.to_owned(),
                    }
                );

                if let Err(e) = write_error {
                    return Some(e);
                }
            }
        }
    }

    None
}

fn main() {
    let matches = App::new("fnd")
        .version("0.3.2")
        .author("Mathew Robinson <mrobinson@praelatus.io>")
        .about(
            "
Find files by regex.

Copyright (C) 2018 Mathew Robinson <mrobinson@praelatus.io>

This program is free software: you can redistribute it and/or modify
it under the terms of the Apache Version 2.0 License

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

You should have recieved a copy of the license with this software if
not you can view it here: https://www.apache.org/licenses/LICENSE-2.0",
        )
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("color").long("color").short("c").help(
            "When specified will highlight matches with ansi \
             term color codes. Note that for large regexes or \
             regexes which match a large portion of text this \
             can negatively affect performance.",
        ))
        .arg(
            Arg::with_name("all")
                .long("all")
                .short("a")
                .help("When specified recurse into hidden directories."),
        )
        .arg(
            Arg::with_name("REGEX")
                .required(true)
                .index(1)
                .help("The REGEX to search for, defaults to fuzzy finding"),
        )
        .arg(
            Arg::with_name("DIRS")
                .multiple(true)
                .help("Where to search for SEARCH")
                .default_value("."),
        )
        .get_matches();

    let search = matches.value_of("REGEX").unwrap();
    let dirs = matches.values_of("DIRS").unwrap().map(|x| {
        if x == "." {
            return env::current_dir().unwrap().to_path_buf();
        }

        PathBuf::from(x)
    });

    let re = format!("(?P<search>{})", search);
    let rgx = Regex::new(&re).unwrap();
    let painter;
    let replacement = if matches.is_present("color") {
        painter = Blue.paint("$search");
        Some(format!("{}", painter))
    } else {
        None
    };

    if let Some(ref err) = find(rgx, dirs, !matches.is_present("all"), &replacement) {
        if err.kind() == io::ErrorKind::BrokenPipe {
            process::exit(0)
        } else {
            println!("ERROR: {}", err);
            process::exit(1)
        }
    }
}
