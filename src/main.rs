/*
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

*/
extern crate pancurses;

use pancurses::{initscr, noecho, Input};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    // read file
    let buffer = fs::read_to_string(&args[1]).unwrap();
    let len = buffer.len();
    if len > 0 {
        // init screen
        let window = initscr();
        // allow scrolling
        window.scrollok(true);
        // disable echo
        noecho();
        let mut i: usize = 0;
        let mut n: usize = 1;
        loop {
            if let Some(ch) = window.getch() {
                // skip ESC
                if ch == Input::Character('\u{1b}') {
                    continue;
                }
                // detect and get indentation
                if &buffer[i..n] == "\n" {
                    while (n + 1) <= len && [" ", "\r", "\t"].contains(&&buffer[n..(n + 1)]) {
                        n += 1;
                    }
                }
                // write to screen
                window.printw(&buffer[i..n]);
                // advance cursor
                i = n % len;
                n = i + 1;
            }
        }
    }
}
