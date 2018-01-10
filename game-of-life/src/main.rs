extern crate ctrlc;
extern crate ncurses;
extern crate game_of_life;

use std::{thread, time};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::process;
use game_of_life::{
    board_as_str,
    calculate_padding,
    mkboard,
    tick
};

fn main() {
    // Clean up ncurses when the program quits with either a SIGINT (Ctrl-C) or
    // a panic.
    let old_panic = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        ncurses::endwin();
        old_panic(info);
    }));
    ctrlc::set_handler(move || {
        ncurses::endwin();
        process::exit(0);
    }).expect("Failed registering ctrl-c handler.");

    let mut board = {
        let mut args = env::args();
        let board_fn = match args.nth(1) {
            Some(x) => x,
            _ => {
                println!("No board given. Falling back on \"boards/gallery.txt\".");
                String::from("boards/gallery.txt")
            },
        };
        let mut board_f = match File::open(&board_fn) {
            Ok(f) => f,
            Err(e) => panic!("Couldn't open \"{}\". Error: {}", board_fn, e),
        };
        let mut board_contents = String::new();
        match board_f.read_to_string(&mut board_contents) {
            Err(e) => panic!("Opened but failed to read \"{}\". Error: {}", board_fn, e),
            _ => {},
        };
        mkboard(board_contents.as_str())
    };

    // This initialization sequence is recommended in the ncurses
    // documentation.
    // http://invisible-island.net/ncurses/man/ncurses.3x.html#h3-Initialization
    ncurses::setlocale(ncurses::LcCategory::all, "");
    let win = ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::nonl();

    let mut winw: i32 = 0;
    let mut winh: i32 = 0;
    ncurses::getmaxyx(win, &mut winh, &mut winw);

    let boardw = board[0].len() as i32;
    let boardh = board.len() as i32;

    let padding = calculate_padding(winw, winh, boardw, boardh);

    loop {
        let b = board_as_str(&board, &padding);
        ncurses::mvprintw(0, 0, b.as_str());
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(100));
        board = tick(&board);
    }
}
