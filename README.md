# fnd [![crates.io](https://img.shields.io/crates/v/fnd.svg)](https://crates.io/crates/fnd)
A simple way to find files.

## Installation
The easiest (and currently only) way to install fnd is using cargo:

```bash
cargo install fnd
```

## Updating

To update fnd (because of a lack of functionality in cargo
[#2082](https://github.com/rust-lang/cargo/issues/2082)) you need to
force a reinstall:

```bash
cargo install --force fnd
```

## Usage

```
fnd 0.3.0
Mathew Robinson <mrobinson@praelatus.io>

Find files by regex.

Copyright (C) 2016 Mathew Robinson <mrobinson@praelatus.io>

This program is free software: you can redistribute it and/or modify
it under the terms of the Apache Version 2.0 License

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

You should have recieved a copy of the license with this software if
not you can view it here: https://www.apache.org/licenses/LICENSE-2.0

USAGE:
	fnd [FLAGS] <REGEX> [DIRS]...

FLAGS:
	-c, --color      When specified will highlight matches with ansi
					 term color codes. Note that for large regexes or
					 regexes which match a large portion of text this
					 can negatively affect performance.
	-h, --help       Prints help information
	-V, --version    Prints version information

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
