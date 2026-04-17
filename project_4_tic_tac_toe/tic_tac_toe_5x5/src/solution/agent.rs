use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;
//use std::{cell::Cell, cmp::{max, min}};

// Your solution solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    fn depth(depth: u32, max_depth: u32) -> bool {
       depth >= max_depth
    }

    fn heuristic(board: &Board) -> i32 {
        let cells = &board.get_cells();
        let n = cells.len();
        let mut score = 0;

        let evaluate = |x_count: i32, o_count: i32| -> i32 {
            match (x_count, o_count) {
                (3, 0) => 100,
                (2, 0) => 10,
                (1, 0) => 1,
                (0, 3) => -100,
                (0, 2) => -10,
                (0, 1) => -1,
                _ => 0,
            }
        };

        let evaluate_window = |cells: &[&Cell; 3]| -> i32 {
            let mut x = 0;
            let mut o = 0;
            for cell in cells {
                match *cell {
                    Cell::X => x += 1,
                    Cell::O => o += 1,
                    _ => {}
                }
            }
            evaluate(x, o)
        };

        for i in 0..n {
            for j in 0..n {
                if j + 2 < n {
                    score += evaluate_window(&[&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]]);
                }
                if i + 2 < n {
                    score += evaluate_window(&[&cells[i][j], &cells[i + 1][j], &cells[i + 2][j]]);
                }
                if i + 2 < n && j + 2 < n {
                    score += evaluate_window(&[&cells[i][j], &cells[i + 1][j + 1], &cells[i + 2][j + 2]]);
                }
                if i + 2 < n && j >= 2 {
                    score += evaluate_window(&[&cells[i][j], &cells[i + 1][j - 1], &cells[i + 2][j - 2]]);
                }
            }
        }

        score
    }

    fn minimax(
        board: &mut Board,
        player: Player,
        depth: u32,
        max_depth: u32,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        }
        if Self::depth(depth, max_depth) {
            return (Self::heuristic(board), 0, 0);
        }

        let maximizing = player == Player::X;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
        let mut best_move = (0, 0);

        for (x, y) in board.moves() {
            board.apply_move((x, y), player);

            let next_player = if player == Player::X {
                Player::O
            } else {
                Player::X
            };

            let (score, _, _) = Self::minimax(board, next_player, depth + 1, max_depth, alpha, beta);
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


// Put your solution here.
impl Agent for SolutionAgent {
    
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let remaining = board.moves().len() as u32;
        let board_size = board.get_cells().len() as u32;

        let max_depth = if board_size == 5 {
            match remaining {
                15..=u32::MAX => 2,
                10..=14 => 3,
                6..=9 => 4,
                _ => remaining,
            }
        } else {
            if remaining >= 6 { 5 } else { remaining }
        };

        SolutionAgent::minimax(board, player, 0, max_depth, i32::MIN, i32::MAX)
    }
}
