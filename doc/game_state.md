I've already done [a little ncurses exploration](./exploring_ncurses.md) so I'm
confident of being able to render the game to the terminal. Now I want to
figure out how to manage the actual game state.

Ncurses will tell me how big the terminal is, so I want to be able to construct
the game board from that information. Cells in the Game of Life respond to
their neighbors and with a two dimensional vector it should be easy enough to
find the neighbors of any cell.

I came back to this after a pretty long hiatus, maybe a couple months. It was
surprisingly easy to get back into Rust. [The standard
library](https://doc.rust-lang.org/std/) has a really nice search and the
examples are great. I think I could write some really cool stuff with structs
and Rust types, but I'm just going to take the easy path and use mutable
vectors and crap -- just get it to compile, you know?

I first wrote a function that would generate an empty game board. I forgot to
have this document open, though, so it was something like this.

    fn new_board(w: u64, h:u64) -> Vec<Vec<bool>> {
        let rows = Vec::new();
        for _ in 0..h {
            let columns = Vec::new();
            for _ in 0..w {
                columns.push(false);
            }
            rows.push(columns);
        }
        rows
    }

Pretty basic, but... Eventually I'll want to be able to take an initial board
state as input and even if I use hard-coded board states, doing that with
nested vectors is annoying. So I switched to taking a string as input and
generating a game board.

    fn mkboard(b: &str) -> Vec<Vec<bool>> {
        let mut rows = Vec::new();

        for line in b.trim().lines() {
            let mut columns = Vec::new();
            for c in line.trim().chars() {
                if c == 'o' {
                    columns.push(true);
                } else {
                    columns.push(false);
                }
            }

            rows.push(columns);
        }

        rows
    }

And I tested it like this:

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn mkboard_test() {
            let b = "
                -----
                --o--
                --o--
                --o--
                -----
            ";

            let board = mkboard(b);
            assert_eq!(board.len(), 5);
            assert_eq!(board[0].len(), 5);
        }
    }

Now I need to render the board. In my ncurses exploration I just used the size
of the terminal, but I might not always want to do that. I could make that the
default and have a configurable window size. In any case, for my testing it
would be easier to be able to set the window to the same size as the initial
configuration. There's a `wresize` function, so it's simple! Okay, so...

I'll convert the board into a one-line string and let ncurses handle the
wrapping.

    type Board = Vec<Vec<bool>>;

    fn board_as_str(board: Board) -> String {
        board.iter().map(|row| row.iter().fold(String::new(), |acc, cell| {
            if *cell {
                acc + "█"
            } else {
                acc + " "
            }
        })).collect::<Vec<String>>().join("")
    }

I used some functional programming features there. I'm not going to explain
them because that code is ass ugly. It reminds me of map and reduce in Python.
Guido wanted to remove them entirely in Python 3; they were moved into their
own module instead. [He felt like other language features made for better
code.](http://www.artima.com/weblogs/viewpost.jsp?thread=98196) I think that's
the case here, too. This would look beautiful in a functional language, but
it's awkward here. I'll do it will nested loops instead.

    fn board_as_str(board: Board) -> String {
        let mut s = String::new();

        for row in board.iter() {
            for cell in row.iter() {
                if *cell {
                    s.push('█');
                } else {
                    s.push(' ');
                }
            }
        }

        s
    }

I should be able to update the game state, too. This is where the rules of the
Game of Life are implemented. Its going to involve checking the state of the
neighbors. The board is infinite, which means that when looking for neighbors
at the edge of the board we should look on the other side as well. The
following function takes the position within a two-dimensional vector and the
width and height of the game board and returns eight positions in the
two-dimensional vector. I'll be able to use those to sum the enabled neighbors.

    fn neighbors(x: u64, y: u64, w: u64, h: u64) -> Vec<(u64, u64)> {
        let left = if x == 0 {
            w - 1
        } else {
            x - 1
        };
        let right = if x == w - 1 {
            0
        } else {
            x + 1
        };
        let top = if y == 0 {
            h - 1
        } else {
            y - 1
        };
        let bottom = if y == h - 1 {
            0
        } else {
            y + 1
        };

        vec![
            (left, top), (x, top), (right, top),
            (left, y), (right, y),
            (left, bottom), (x, bottom), (right, bottom),
        ]
    }

The entire board is updated all at once (a discrete moment). According to the
Wikipedia article this is sometimes called a "tick". Here we go!

    pub fn tick(b: &Board) -> Board {
        let bh = b.len();
        let bw = b[0].len();
        let mut rows = Vec::new();

        for y in 0..bh {
            let mut columns = Vec::new();

            for x in 0..bw {
                let mut num_living_neighbors = 0;
                for (nx, ny) in neighbors(x, y, bw, bh) {
                    if b[ny][nx] {
                        num_living_neighbors += 1;
                    }
                }

                let living = b[y][x];
                if living {
                    if num_living_neighbors < 2 {
                        columns.push(false);
                    } else if num_living_neighbors < 4 {
                        columns.push(true);
                    } else {
                        columns.push(false);
                    }
                } else {
                    if num_living_neighbors == 3 {
                        columns.push(true);
                    } else {
                        columns.push(false);
                    }
                }
            }

            rows.push(columns);
        }

        rows
    }

This goes through every cell, counts the number of living neighbors, and then
applies [the
rules](https://en.m.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules). It
returns a new game board. And with that I'm able to throw together this main
function that has a game board with [these example still lifes and
oscillators](https://en.m.wikipedia.org/wiki/Conway%27s_Game_of_Life#Examples_of_patterns).

    extern crate ncurses;
    extern crate game_of_life;

    use std::{cmp, thread, time};
    use game_of_life::{board_as_str, mkboard, tick};

    fn main() {
        let win = ncurses::initscr();

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
