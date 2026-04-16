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
}

// Put your solution here.
impl Agent for SolutionAgent {

    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let depth = board.moves().len();
        if board.game_over() || SolutionAgent::depth(depth as u32, 5){
            return (board.score(), 0, 0);      
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

            let (score, _, _) = SolutionAgent::solve(&mut new_board, next_player, _time_limit);

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
