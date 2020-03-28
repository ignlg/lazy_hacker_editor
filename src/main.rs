use pancurses::*;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

// Lazy Hacker Editor
/// Copyright (C) 2020 Ignacio Lago
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program. If not, see <https://www.gnu.org/licenses/>.

#[derive(StructOpt, Debug)]
#[structopt(name = "Lazy Hacker Editor (lhe)")]
struct Opt {
    /// Set chars per keypress
    #[structopt(short, long, default_value = "1")]
    cps: usize,

    /// Text color (0-7):
    #[structopt(long, default_value = "2")]
    color: u32,

    // TODO: Activate line per keypress, ignores -c, --cps
    // #[structopt(short, long)]
    // lps: bool,
    /// Highlight some common code characters
    #[structopt(short = "l", long)]
    highlight: bool,

    /// Highlight color (0-7):
    #[structopt(long, default_value = "7")]
    hcolor: u32,

    /// Files to process
    #[structopt(required = true, name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

const HIGHLIGHT: [char; 36] = [
    '(', ')', ',', ';', ':', '.', '+', '*', '\'', '"', '-', '=', '{', '}', '[', ']', '~', '/',
    '\\', '&', '?', '<', '>', '|', '`', '!', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

const COLOR_TABLE: [i16; 8] = [
    COLOR_WHITE,
    COLOR_BLUE,
    COLOR_GREEN,
    COLOR_CYAN,
    COLOR_RED,
    COLOR_MAGENTA,
    COLOR_YELLOW,
    COLOR_WHITE,
];

fn main() {
    // opts
    let opt = Opt::from_args();
    // init screen
    let window = initscr();
    // allow scrolling
    window.scrollok(true);
    // disable echo
    noecho();
    // colors?
    if has_colors() {
        start_color();
    }
    for (i, color) in COLOR_TABLE.iter().enumerate() {
        init_pair(i as i16, *color, COLOR_BLACK);
    }
    window.attrset(COLOR_PAIR(opt.color % 8) | A_NORMAL);
    // go
    let mut file_idx = 0;
    while file_idx < opt.files.len() {
        // read file
        let buffer = fs::read_to_string(&opt.files[file_idx]).unwrap();
        let len = buffer.len();
        if len > 0 {
            let mut i: usize = 0;
            let mut n: usize = opt.cps;
            while i < len {
                if let Some(ch) = window.getch() {
                    match ch {
                        // skip ESC
                        Input::Character('\u{1b}') => continue,
                        Input::KeyResize => {
                            resize_term(0, 0);
                            continue;
                        }
                        _ => {}
                    };
                    // detect and get indentation
                    if &buffer[i..n] == "\n" {
                        while (n + 1) <= len && [" ", "\r", "\t"].contains(&&buffer[n..(n + 1)]) {
                            n += 1;
                        }
                    }
                    // highlight
                    if opt.highlight {
                        let str_to_add = &buffer[i..n];
                        for c in str_to_add.chars() {
                            if HIGHLIGHT.contains(&c) {
                                window.attrset(COLOR_PAIR(opt.hcolor % 8) | A_BOLD);
                            } else {
                                window.attrset(COLOR_PAIR(opt.color % 8) | A_NORMAL);
                            }
                            window.addch(c);
                        }
                    } else {
                        // write to screen
                        window.addstr(&buffer[i..n]);
                    }
                    // advance cursor
                    i = n;
                    if len - i > opt.cps {
                        n = i + opt.cps;
                    } else {
                        n = len;
                    }
                }
            }
        }
        file_idx = (file_idx + 1) % opt.files.len();
    }
    endwin();
}
