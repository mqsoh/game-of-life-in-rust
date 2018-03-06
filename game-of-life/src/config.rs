extern crate getopts;

static USAGE: &str = r###"
NAME
        game_of_life - Plays the Game of Life in your terminal.

SYNOPSIS
        game_of_life [BOARD] [options]

DESCRIPTION
        The BOARD is a file with the lines and columns of a game board. If the
        starting state of the cell is on, then it should have a "0". An empty
        cell can be any other character. I used a "." in my example boards. The
        default board is a gallery of still lifes and oscillators.

        The board is indepedent of the window size. By default it will be
        centerd to the viewport whether it's larger or smaller. For boards that
        are smaller than the window in width or height, the anchor flag will
        orient the board to one of the four corners and resize it to be the
        same size as the window.

        -a, --anchor ALIGNMENT
                Two characters that represent the vertical and horizontal
                alignment. The top left is "tl", the center is "cc", and the
                center of the right side is "cr". Valid characters are: c, t,
                b, r, and l for center, top, bottom, right, left.
        -h, --help
                This message.
"###;

#[derive(Debug)]
pub enum Anchor {
    None,
    TL, TC, TR,
    CL, CC, CR,
    BL, BC, BR,
}

#[derive(Debug)]
pub struct Config {
    pub anchor: Anchor,
    pub starting_board: String,
}

pub fn from_args(args: Vec<String>) -> Result<Config, String> {
    let mut options = getopts::Options::new();
    options.optopt("a", "anchor", "", "ALIGNMENT");
    options.optflag("h", "help", "");

    let matches = match options.parse(&args[1..]) {
        Err(e) => {
            let e = format!("Failed parsing command line arguments. Error: {}", e.to_string());
            return Err(String::from(e.as_str()))
        },
        Ok(m) => m,
    };

    if matches.opt_present("h") {
        return Err(String::from(USAGE));
    }

    return Ok(Config{
        starting_board: match matches.free.len() {
            0 => String::from("boards/gallery.txt"),
            _ => matches.free[0].clone(),
        },

        anchor: match matches.opt_str("a") {
            None => Anchor::None,
            Some(ref s) => {
                if s == "tl" {
                    Anchor::TL
                } else if s == "tc" {
                    Anchor::TC
                } else if s == "tr" {
                    Anchor::TR
                } else if s == "cl" {
                    Anchor::CL
                } else if s == "cc" {
                    Anchor::CC
                } else if s == "cr" {
                    Anchor::CR
                } else if s == "bl" {
                    Anchor::BL
                } else if s == "bc" {
                    Anchor::BC
                } else if s == "br" {
                    Anchor::BR
                } else {
                    let e = format!("Invalid anchor value. Expected: tc, tr, cl, cc, cr, bl, bc, or br but I got \"{}\".", s);
                    return Err(String::from(e.as_str()))
                }
            },
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_configuration() {
        match from_args(vec![String::from("/path/to/executable"), String::from("--anchor"), String::from("fouled")]) {
            Ok(_) => assert!(false, "I gave an invalid anchor value, so this shouldn't have passed."),
            Err(_) => {},
        }
    }

    #[test]
    fn help() {
        match from_args(vec![String::from("/path/to/executable"), String::from("--help")]) {
            Ok(_) => assert!(false, "I asked for help, so I should have got a error back with usage info."),
            Err(usage) => assert_eq!(usage, String::from(USAGE)),
        }
    }
}
