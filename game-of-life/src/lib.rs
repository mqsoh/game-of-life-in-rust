type Board = Vec<Vec<bool>>;

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

pub fn board_as_str(board: &Board) -> String {
    let mut s = String::new();

    for row in board.iter() {
        for cell in row.iter() {
            if *cell {
                s.push_str("█");
            } else {
                s.push_str("░");
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
        assert_eq!(board_as_str(board), "       0    0    0       ");
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
}
