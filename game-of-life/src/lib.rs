type Board = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Padding {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl std::cmp::PartialEq for Padding {
    fn eq(&self, other: &Padding) -> bool {
        self.top == other.top && self.right == other.right && self.bottom == other.bottom && self.left == other.left
    }
}

pub fn calculate_padding(window_width: i32, window_height: i32, board_width: i32, board_height: i32) -> Padding {
    let diffw = window_width - board_width;
    let diffh = window_height - board_height;

    let diff_halfw = diffw as f32 / 2.0;
    let diff_halfh = diffh as f32 / 2.0;

    return Padding {
        top: diff_halfh.floor() as i32,
        right: diff_halfw.ceil() as i32,
        bottom: diff_halfh.ceil() as i32,
        left: diff_halfw.floor() as i32,
    }
}

pub fn mkboard(b: &str) -> Board {
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

pub fn board_as_str(board: &Board, padding: &Padding) -> String {
    let mut s = String::new();

    let lines = board.len() as i32;
    let cols = board[0].len() as i32;

    for line in 0 - padding.top..lines + padding.bottom {
        for col in 0 - padding.left..cols + padding.right {
            if line < 0 || line >= lines || col < 0 || col >= cols {
                s.push_str(" ");
            } else if board[line as usize][col as usize] {
                s.push_str("0");
            } else {
                s.push_str(" ");
            }
        }
    }

    s
}

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_BOARD: &str = "
        -----
        --o--
        --o--
        --o--
        -----
    ";

    #[test]
    fn mkboard_test() {
        let board = mkboard(TEST_BOARD);
        assert_eq!(board.len(), 5);
        assert_eq!(board[0].len(), 5);
    }

    #[test]
    fn board_as_str_test() {
        let board = mkboard(TEST_BOARD);
        assert_eq!(board_as_str(&board, &Padding{ top: 0, right: 0, bottom: 0, left: 0}), "       0    0    0       ");
        assert_eq!(board_as_str(&board, &Padding{ top: 1, right: 1, bottom: 1, left: 1}), "                 0      0      0                 ");
        assert_eq!(board_as_str(&board, &Padding{ top: -1, right: -1, bottom: -1, left: -1}), " 0  0  0 ");
    }

    #[test]
    fn neighbors_test() {
        // This is the board, getting the neighbors of the cell marked "x".
        //
        //     --
        //     -x
        assert_eq!(neighbors(1, 1, 2, 2), vec![
            (0, 0), (1, 0), (0, 0),
            (0, 1), (0, 1),
            (0, 0), (1, 0), (0, 0),
        ]);

        // This is the board, getting the neighbors of the cell marked "x".
        //
        //     x-
        //     --
        assert_eq!(neighbors(0, 0, 2, 2), vec![
            (1, 1), (0, 1), (1, 1),
            (1, 0), (1, 0),
            (1, 1), (0, 1), (1, 1),
        ]);

        // This is the board, getting the neighbors of the cell marked "x".
        //
        //     ---
        //     -x-
        //     ---
        assert_eq!(neighbors(1, 1, 3, 3), vec![
            (0, 0), (1, 0), (2, 0),
            (0, 1), (2, 1),
            (0, 2), (1, 2), (2, 2),
        ]);
    }

    #[test]
    fn tick_test() {
        let initial_state = vec![
            vec![false, false, false, false, false],
            vec![false, false, true , false, false],
            vec![false, false, true , false, false],
            vec![false, false, true , false, false],
            vec![false, false, false, false, false],
        ];

        let second_state = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true , true , true , false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];

        assert_eq!(tick(&initial_state), second_state);
        assert_eq!(tick(&second_state), initial_state);
    }

    #[test]
    fn test_calculate_padding() {
        assert_eq!(
            Padding { top: 1, right: 1, bottom: 1, left: 1 },
            calculate_padding(3, 3, 1, 1)
        );

        assert_eq!(
            Padding { top: 1, right: 2, bottom: 2, left: 1 },
            calculate_padding(4, 4, 1, 1)
        );

        assert_eq!(
            Padding { top: -1, right: -1, bottom: -1, left: -1 },
            calculate_padding(1, 1, 3, 3)
        );

        assert_eq!(
            Padding { top: -2, right: -1, bottom: -1, left: -2 },
            calculate_padding(1, 1, 4, 4)
        );
    }
}
