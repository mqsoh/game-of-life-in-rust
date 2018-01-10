extern crate ctrlc;
extern crate ncurses;
extern crate game_of_life;

use std::{thread, time};
use std::panic;
use std::process;
use game_of_life::{board_as_str, calculate_padding, mkboard, tick};

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

    // This initialization sequence is recommended in the ncurses
    // documentation.
    // http://invisible-island.net/ncurses/man/ncurses.3x.html#h3-Initialization
    ncurses::setlocale(ncurses::LcCategory::all, "");
    let win = ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::nonl();

    let mut board = mkboard("
        ---------------------------------------------
        -oo---oo----oo---oo----o---------------------
        -oo--o--o--o--o--o-o--o-o--------------------
        ------oo----o-o---o----o---------------------
        -------------o-------------------------------
        ---------------------------------------------
        ---------------------------------------------
        --o-----o---oo-------------------------o-----
        --o---o--o--oo-------ooo---ooo---------o-----
        --o---o--o----oo----------------------ooo----
        -------o------oo---o----o-o----o-------------
        -------------------o----o-o----o-------------
        -------------------o----o-o----o------ooo----
        ---------------------ooo---ooo---------o-----
        ---------------------------------------o-----
        ---------------------ooo---ooo---------o-----
        -------------------o----o-o----o-------o-----
        -------------------o----o-o----o------ooo----
        -------------------o----o-o----o-------------
        ---------------------------------------------
        ---------------------ooo---ooo--------ooo----
        ---------------------------------------o-----
        ---------------------------------------o-----
        ---------------------------------------------
    ");

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
