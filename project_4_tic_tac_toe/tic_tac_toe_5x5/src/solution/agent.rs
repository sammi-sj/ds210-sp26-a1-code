use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use std::cmp::{max, min};

// Your solution solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    fn depth(depth: u32, max_depth: u32) -> bool {
       depth >= max_depth
}
fn heuristic(board: &Board) -> i32 {
        board.score()
}
fn minimax(board: &mut Board, player: Player, depth: u32, max_depth: u32) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);      
        }
        if Self::depth(depth, max_depth) {
            return (Self::heuristic(board),0,0);
        }
        let mut best_move = (0,0);
        let maximizing = player == Player::X; //true if player is X, false if player is O
        let mut best_score = //best score is max if maximizing, min if minimizing
            if maximizing {
                i32::MIN 
            }
            else {
            i32::MAX
            };
        
        for (x,y) in board.moves() {
            let mut new_board = board.clone();
            new_board.apply_move((x, y), player);

            let next_player = 
                match player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };

            let (score, _, _) = Self::minimax(&mut new_board, next_player, depth +1, max_depth);

            if maximizing && score > best_score {
                best_score = score;
                best_move = (x, y);
            }

            if !maximizing && score < best_score {
                best_score = score;
                best_move = (x, y);
            }    
        }
    return (best_score, best_move.0, best_move.1);
    }
}
// Put your solution here.
impl Agent for SolutionAgent {
    
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let remaining = board.moves().len() as u32;
        let max_depth = if remaining <= 9 {
            remaining   // full search for 3x3
        } else {
        3           // limit for 5x5
        };
       SolutionAgent::minimax(board, player, 0, max_depth)
    }
}
