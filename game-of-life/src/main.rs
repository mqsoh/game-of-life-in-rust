extern crate ncurses;

use std::{thread, time};

fn main() {
    let win = ncurses::initscr();
    let mut maxx: i32 = 0;
    let mut maxy: i32 = 0;

    ncurses::getmaxyx(win, &mut maxy, &mut maxx);

    loop {
        ncurses::mvprintw(0, 0, "-".repeat((maxx * maxy) as usize).as_str());

        ncurses::refresh();

        thread::sleep(time::Duration::from_secs(1));

        ncurses::mvprintw(0, 0, ".".repeat((maxx * maxy) as usize).as_str());

        ncurses::refresh();

        thread::sleep(time::Duration::from_secs(1));
    }

    ncurses::getch();
    ncurses::endwin();
}
