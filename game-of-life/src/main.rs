extern crate ctrlc;
extern crate getopts;
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
    let usage = r###"
NAME
        game_of_life - Plays the Game of Life in your terminal.

SYNOPSIS
        game_of_life [BOARD] [options]

DESCRIPTION
        The BOARD is a file with the lines and columns of a game board. If the
        starting state of the cell is on, then it should have a "0". An empty
        cell can be any other character. I used a "." in my example boards. It
        defaults to a gallery of still lifes and oscillators.

        The board is indepedent of the window size. By default it will be
        centerd to the viewport whether it's larger or smaller. For boards that
        are smaller then the window, the anchor flag will orient the board to
        one of the four corners and resize it to be the same size as the
        window.

        -a, --anchor ALIGNMENT
                tl (top left), tr (top right), bl (bottom left), or br (bottom right)
"###;
    let args: Vec<String> = env::args().collect();
    let mut options = getopts::Options::new();
    options.optopt("a", "anchor", "", "ALIGNMENT");
    options.optflag("h", "help", "");
    let matches = match options.parse(&args[1..]) {
        Err(e) => panic!("Failed parsing command line arguments. Error: {}", e.to_string()),
        Ok(m) => m,
    };

    if matches.opt_present("h") {
        println!("{}", usage);
        return;
    }

    let mut board = {
        let board_fn = if matches.free.is_empty() {
            println!("No board given. Falling back on \"boards/gallery.txt\".");
            String::from("boards/gallery.txt")
        } else {
            matches.free[0].clone()
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

    // Resize the board if the anchor flag is given.
    if matches.opt_present("a") && (boardw < winw || boardh < winh) {
        let corner = matches.opt_str("a");
        println!("{:?}", corner);
    }

    loop {
        let b = board_as_str(&board, &padding);
        ncurses::mvprintw(0, 0, b.as_str());
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(100));
        board = tick(&board);
    }
}
