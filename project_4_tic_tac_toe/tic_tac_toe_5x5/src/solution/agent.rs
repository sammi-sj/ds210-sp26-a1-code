use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;

pub struct SolutionAgent {}

impl SolutionAgent {
    #[inline]
    fn evaluate_window(cells: &[&Cell; 3]) -> i32 {
        let mut x_count = 0;
        let mut o_count = 0;
        let mut walls = 0;

        for cell in cells {
            match *cell {
                Cell::X => x_count += 1,
                Cell::O => o_count += 1,
                Cell::Wall => walls += 1,
                _ => {}
            }
        }

        // blocked line
        if walls > 0 {
            return 0;
        }

        if x_count > 0 && o_count == 0 {
            return match x_count {
                3 => -6000,
                2 => -800,
                1 => -50,
                _ => 0,
            };
        }

        if o_count > 0 && x_count == 0 {
            return match o_count {
                3 => 6000,
                2 => 800,
                1 => 50,
                _ => 0,
            };
        }

        0
    }

    fn heuristic(board: &Board) -> i32 {
        let score = board.score() * 1000;
        if score > 0 { return 1000; }
        if score < 0 { return -1000; }
        if score == 0 && board.moves().is_empty() { return 0; }

        let cells = &board.get_cells();
        let n = cells.len();
        let mut eval = 0;

        // all windows
        for i in 0..n {
            for j in 0..n {

                if j + 2 < n {
                    eval += Self::evaluate_window(
                        &[&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]]
                    );
                }

                if i + 2 < n {
                    eval += Self::evaluate_window(
                        &[&cells[i][j], &cells[i + 1][j], &cells[i + 2][j]]
                    );
                }

                if i + 2 < n && j + 2 < n {
                    eval += Self::evaluate_window(
                        &[&cells[i][j], &cells[i + 1][j + 1], &cells[i + 2][j + 2]]
                    );
                }

                if i + 2 < n && j >= 2 {
                    eval += Self::evaluate_window(
                        &[&cells[i][j], &cells[i + 1][j - 1], &cells[i + 2][j - 2]]
                    );
                }
            }
        }

        // center control
        if n > 3 {
            let center = n / 2;
            for i in center.saturating_sub(1)..=(center + 1).min(n - 1) {
                for j in center.saturating_sub(1)..=(center + 1).min(n - 1) {
                    match cells[i][j] {
                        Cell::X => eval -= 5,
                        Cell::O => eval += 5,
                        _ => {}
                    }
                }
            }
        }

        eval
    }

    fn count_threats(board: &Board) -> i32 {
        let cells = board.get_cells();
        let n = cells.len();
        let mut threats = 0;

        for i in 0..n {
            for j in 0..n {
                if j + 2 < n {
                    let window = [&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]];
                    let mut x = 0;
                    let mut o = 0;

                    for c in window {
                        match c {
                            Cell::X => x += 1,
                            Cell::O => o += 1,
                            _ => {}
                        }
                    }

                    if x == 2 && o == 0 {
                        threats += 1;
                    }
                }
            }
        }

        threats
    }

    fn compute_depth(board: &Board) -> u32 {
        let moves_left = board.moves().len();

        if moves_left > 20 {
            4
        } else if moves_left > 12 {
            5
        } else if moves_left > 6 {
            6
        } else {
            8
        }
    }

    
    fn minimax(
        board: &mut Board,
        player: Player,
        depth: u32,
        max_depth: u32,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, usize, usize) {

        // depth limit
        if depth == max_depth {
            let mut h = Self::heuristic(board);

            // ⭐ KEY FIX: O is defensive → punish X threats more
            if player == Player::O {
                h -= Self::count_threats(board) * 300;
            }

            return (h, 0, 0);
        }

        if board.game_over() {
            let score = board.score();
            let final_score =
                if score > 0 { 1000 }
                else if score < 0 { -1000 }
                else { 0 };

            return (final_score, 0, 0);
        }

        let mut moves = board.moves();
        if moves.is_empty() {
            return (0, 0, 0);
        }

        let cells = board.get_cells();
        let n = cells.len();

        let maximizing = player == Player::X;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
        let mut best_move = (0, 0);

        for (x, y) in moves {
            board.apply_move((x, y), player);

            let next_player = if player == Player::X {
                Player::O
            } else {
                Player::X
            };

            let (score, _, _) = Self::minimax(
                board,
                next_player,
                depth + 1,
                max_depth,
                alpha,
                beta,
            );

            board.undo_move((x, y), player);

            if maximizing {
                if score > best_score {
                    best_score = score;
                    best_move = (x, y);
                }
                alpha = alpha.max(score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = (x, y);
                }
                beta = beta.min(score);
            }

            if alpha >= beta {
                break;
            }
        }

        (best_score, best_move.0, best_move.1)
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let board_size = board.get_cells().len();

        let max_depth = match board_size {
            3 => 9,
            5 => Self::compute_depth(board),
            _ => 4,
        };

        Self::minimax(board, player, 0, max_depth, i32::MIN, i32::MAX)
    }
}