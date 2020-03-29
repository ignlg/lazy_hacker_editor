use pancurses::{
    endwin, has_colors, init_pair, initscr, noecho, resize_term, start_color, Input, Window,
    A_BOLD, A_NORMAL, COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_PAIR,
    COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines};
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
    color: usize,

    /// Activate line per keypress, ignores -c, --cps
    #[structopt(long)]
    lps: bool,

    /// Highlight some common code characters
    #[structopt(short = "l", long)]
    highlight: bool,

    /// Highlight color (0-7):
    #[structopt(long, default_value = "7")]
    hcolor: usize,

    /// Files to process
    #[structopt(required = true, name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

const HIGHLIGHT_CHARS: [char; 36] = [
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

const COLOR_TEXT: u32 = 1;
const COLOR_HIGHLIGHT: u32 = 2;

/// Initializes pancurses window
fn init_window(opt: &Opt) -> Window {
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
    init_pair(COLOR_TEXT as i16, COLOR_TABLE[opt.color % 8], COLOR_BLACK);
    init_pair(
        COLOR_HIGHLIGHT as i16,
        COLOR_TABLE[opt.hcolor % 8],
        COLOR_BLACK,
    );
    window.attrset(COLOR_PAIR(COLOR_TEXT) | A_NORMAL);
    window
}

/// Adds text to a pancurses window
fn add_text(slice: &str, window: &Window, highlight: bool) {
    if highlight {
        // char by char, highlighting
        for c in slice.chars() {
            if HIGHLIGHT_CHARS.contains(&c) {
                window.attrset(COLOR_PAIR(COLOR_HIGHLIGHT) | A_BOLD);
            } else {
                window.attrset(COLOR_PAIR(COLOR_TEXT) | A_NORMAL);
            }
            window.addch(c);
        }
    } else {
        // write the whole slice
        window.addstr(slice);
    }
}

/// Reads a keypress and writes code on a loop
fn lazy_editor_by_chars(buffer: &str, window: &Window, opt: &Opt) {
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
                // detect end of line and continue until end of next indentation
                if &buffer[i..n] == "\n" {
                    while (n + 1) <= len && [" ", "\r", "\t"].contains(&&buffer[n..(n + 1)]) {
                        n += 1;
                    }
                }
                add_text(&buffer[i..n], &window, opt.highlight);
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
}

fn lazy_editor_by_lines(lines: Lines<BufReader<File>>, window: &Window, opt: &Opt) {
    for line in lines {
        if let Ok(mut line) = line {
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
                line.push_str("\n");
                add_text(&line, &window, opt.highlight);
            }
        }
    }
}

fn main() {
    // opts
    let opt = Opt::from_args();
    // init screen
    let window = init_window(&opt);
    // files loop
    let mut file_idx = 0;
    let mut read_success = false;
    while file_idx < opt.files.len() {
        if opt.lps {
            // read file
            if let Ok(file) = File::open(&opt.files[file_idx]) {
                read_success = true;
                let buffer = BufReader::new(file).lines();
                // consume with lazy editor
                lazy_editor_by_lines(buffer, &window, &opt);
            }
        } else {
            // read file
            if let Ok(buffer) = read_to_string(&opt.files[file_idx]) {
                read_success = true;
                // consume with lazy editor
                lazy_editor_by_chars(&buffer, &window, &opt);
            }
        }
        if read_success {
            // next file, endless
            file_idx = (file_idx + 1) % opt.files.len();
        } else {
            // next file
            file_idx += 1;
        }
    }
    // end ncurses window
    endwin();
}
