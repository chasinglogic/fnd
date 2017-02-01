# fnd [![crates.io](https://img.shields.io/crates/v/fnd.svg)](https://crates.io/crates/fnd)
A simple way to find files.

## Installation
The easiest (and currently only) way to install fnd is using cargo:

```bash
cargo install fnd
```
 
## Updating
To update fnd (because of a lack of functionality in cargo 
[#2082](https://github.com/rust-lang/cargo/issues/2082)) you need to uninstall
then install it again:

```bash
cargo uninstall fnd
cargo install fnd
```

## Usage

```
fnd, a simpler way to find files 0.2.0
Mathew Robinson <mrobinson@praelatus.io>
Finds files

USAGE:
    fnd [FLAGS] <REGEX> [ARGS]

FLAGS:
    -n, --no_color    When specified will not highlight matches with ansi
                      term color codes this is useful when piping output
    -h, --help        Prints help information
    -V, --version     Prints version information

ARGS:
    <REGEX>      The REGEX to search for, defaults to fuzzy finding
    <DIRS>...    Where to search for SEARCH [default: .]
```

### Quick Start 

Assuming that your using bash and want to find all rust source files in your
`~/Code` directory you simply run:

```bash
fnd \\.rs\$ ~/Code
```

That's it! The first argument to fnd is a regex and will be interpreted
literally, so make sure you escape dots and the like or else they will match
any character.

You can optionally supply multiple directories and fnd will search through all
of them. Expanding our example:

```bash
fnd \\.mp3\$ ~/Code ~/Documents ~/Music
```

This will search for mp3's in `~/Code`, `~/Documents`, and `~/Music`.

**Note:** fnd is always recursive and will default to the current directory.

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. :fire: Submit a pull request :D :fire:

All pull requests should go to the develop branch not master. Thanks!

## License

This code is distributed under the Apache 2.0 License.

```
Copyright 2017 Mathew Robinson

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
