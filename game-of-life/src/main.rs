extern crate ncurses;
extern crate game_of_life;

use std::{thread, time};
use game_of_life::{board_as_str, mkboard, tick};

fn main() {
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

    ncurses::wresize(win, board.len() as i32, board[0].len() as i32);

    loop {
        let b = board_as_str(&board);
        ncurses::mvprintw(0, 0, b.as_str());
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(100));
        board = tick(&board);
    }
}
