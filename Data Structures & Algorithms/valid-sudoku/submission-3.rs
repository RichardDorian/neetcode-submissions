impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();

        for row in 0..9 {
            for col in 0..9 {
                println!("{}", board[row][col]);
                if board[row][col] == '.' {
                    continue;
                }

                if !set.insert(board[row][col]) {
                    return false;
                }
            }

            println!("----");

            // For each row we process we clear the previous known numbers
            set.clear();
        }

        for col in 0..9 {
            for row in 0..9 {
                if board[row][col] == '.' {
                    continue;
                }

                if !set.insert(board[row][col]) {
                    return false;
                }
            }

            // For each column we process we clear the previous known numbers
            set.clear();
        }

        for square_row in 0..3 {
            for square_col in 0..3 {
                for row in 0..3 {
                    for col in 0..3 {
                        let val = board[row + (3 * square_row)][col + (3 * square_col)];

                        if val == '.' {
                            continue;
                        }

                        if !set.insert(val) {
                            return false;
                        }
                    }
                }

                // After each square we clear known members
                set.clear();
            }
        }

        true
    }
}