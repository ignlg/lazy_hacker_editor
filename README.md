# Lazy Hacker Editor

[![Build Status](https://travis-ci.org/ignlg/lazy_hacker_editor.svg?branch=master)](https://travis-ci.org/ignlg/lazy_hacker_editor) [![Build status](https://ci.appveyor.com/api/projects/status/wc7p5th4c7onpmo4/branch/master?svg=true)](https://ci.appveyor.com/project/ignlg/lazy-hacker-editor/branch/master)

_Write code already written._

Fake code editor that writes real code, probably yours, no matter which key you stroke.

> Nothing beats the eficiency of writing code already written.
> _— me, circa 2020_

## Usage

```
USAGE:
    lhe [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -h, --help
            Prints help information

    -l, --highlight
            Highlight some common code characters

    -V, --version
            Prints version information

OPTIONS:
        --color <color>
            Text color (0-7): [default: 2]

    -c, --cps <cps>
            Set chars per keypress [default: 1]

        --hcolor <hcolor>
            Highlight color (0-7): [default: 7]

ARGS:
    <FILE>...
            Files to process
```

**Example**

```
lhe -l -c 5 src/main.rs
```

Exit with `^C`. I hope you know what this means, _"""hacker"""._

## Roadmap

### v0.1.0

- [x] read file
- [x] write a char on key down
- [x] ignore ESC

### v0.5.0

- [x] read from multiple files
- [x] detect term resize
- [x] opt `-V, --version`
- [x] opt `-h, --help`
- [x] opt chars per stroke `-c, --cps N`
- [x] opt color `--color`
- [x] rudimentary code highlight (common symbols)
- [x] opt highlight `-l, --highlight`
- [x] opt highlight color `--hcolor`

### v0.6.0

- [x] opt use line per stroke: `--lps`
- [x] control read errors
- [x] control Ctrl+C clean exit

### Backlog

- [ ] add gif to README
- [ ] add a logo
- [ ] get rid of ncurses?

## License

Lazy Hacker Editor
Copyright (C) 2020 Ignacio Lago

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
