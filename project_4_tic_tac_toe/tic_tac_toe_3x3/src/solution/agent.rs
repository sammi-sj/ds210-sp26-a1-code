use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        if board.game_over() {
            if board.score() == 1 {
                println!("X wins");
                return board.moves();
            }
            else if board.score() == &-1 {
                println!("O wins");
                return board.moves();
            }
            else if board.score() == 0{
                println!("Draw");
                return board.moves();
            }
        }
        println!("There are {} moves left", board.moves());
        board.clone().apply_move(location, player);

        if player == Player::X {
            score = SolutionAgent::solve(board, Player::X, _time_limit);
        }
        else if player == Player::O {
            score = SolutionAgent::solve(board, Player::O, _time_limit);
        }
    }
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        unimplemented!("Not yet implemented")
    }
}
