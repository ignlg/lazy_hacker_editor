# Lazy Hacker Editor

_Write code already written._

> Nothing beats the eficiency of writing code already written.
> _— me, circa 2020_

## Usage

```
lhe {path to file}
```

Example, in case you need one:

```
lhe "src/fancy_code_file.rs"
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

### Backlog

- [ ] param to setup line per stroke: --lps
- [ ] add gif to README
- [ ] add a logo

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
