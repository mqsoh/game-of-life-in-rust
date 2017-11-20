I've never written anything with ncurses before and this project is meant as a
Rust introduction. There's a library called
[Cursive](https://github.com/gyscos/Cursive) which is "focused on ease of use",
however I found it difficult. As a beginner in both the language and ncurses, I
wasn't sure which documentation to look at. I settled on the ["very thin
wrapper around the ncurses TUI lib"](https://github.com/jeaye/ncurses-rs).

I didn't write this while I was doing my exploration, so I don't recall what
other code I wrote. At first I thought the I needed to be able to write to
arbitrary locations on the screen and I did a sample program where I did that.
However, the whole point of the Game of Life is that there are "ticks" where
every cell is updated. What I actually need to do is redraw the entire screen.

There's two places I looked at documentation. [The official ncurses site has
some, of course!](https://www.gnu.org/software/ncurses/ncurses.html) But that
doesn't help with the actually Rust types. Once I figured out how the functions
are abbreviated, I could infer what I neeed by looking at the ncurses-rs
documentation. You can download the docs for the specific version of code
you're using! This is an awesome feature of cargo. I ran

    cargo doc

...and I could access the ncurses crate in the project directory
`game-of-life/target/doc/game_of_life/index.html`.

Anyway, this is the bit of code I came up with. It cycles the entire screen
between `-` characters and `.` characters.

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

I first figure out how big the screen is, then I move the cursor to the top
left of the screen and print out the number of `-` characters to fill up the
screen. The `"-".repeat((maxx * maxy) as usize).as_str()` expression creates
the string to fill up the screen. Initially I tried adding line breaks but
ncurses wraps the text like you'd expect. After that I redraw the screen, sleep
for one second, and then do the same thing with the `.` character. The `getch`
and `endwin` calls aren't actually needed in this program (because it's looping
infinitely) but I think they're needed for a real program.
