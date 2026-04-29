use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;

// Your solution solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    
    #[inline]
    fn evaluate(x_count: i32, o_count: i32) -> i32 {    
            match (x_count, o_count) {
                (5, 0) => 10000,
                (4, 0) => 10000,
                (3, 0) => 1000,
                (2, 0) => 100,
                (1, 0) => 10,
                (0, 5) => -10000,
                (0, 4) => -10000,
                (0, 3) => -1000,
                (0, 2) => -100,
                (0, 1) => -10,
                _ => 0,
            }
        }
    
        #[inline]
    fn evaluate_window(cells: &[&Cell; 3]) -> i32 {
            let mut x_count = 0;
            let mut o_count = 0;
            for cell in cells {
                match *cell {
                    Cell::X => x_count += 1,
                    Cell::O => o_count += 1,
                    Cell::Empty => {},
                    _ => {}
                }
            }
            Self::evaluate(x_count, o_count)
        }

    fn heuristic(board: &Board) -> i32 {
        let score = board.score() * 1000;
        if score > 0 {return 1000};
        if score < 0 {return -1000};
        if score == 0 && board.moves().is_empty() {return 0};
        
        let cells = &board.get_cells();
        let n = cells.len();
        let mut eval = 0;

        for i in 0..n { 
            for j in 0..n {
                if j + 2 < n { //horizontal windows
                    eval += Self::evaluate_window(&[&cells[i][j], 
                                                    &cells[i][j + 1], 
                                                    &cells[i][j + 2]]);
                }
                if i + 2 < n { //vertical windows
                    eval += Self::evaluate_window(&[&cells[i][j], 
                                                    &cells[i + 1][j], 
                                                    &cells[i + 2][j]]);
                }
                if i + 2 < n && j + 2 < n { //diagonal top left to bottom right
                    eval += Self::evaluate_window(&[&cells[i][j], 
                                                    &cells[i + 1][j + 1], 
                                                    &cells[i + 2][j + 2]]);
                }
                if i + 2 < n && j >= 2 { //diagonal top right to bottom left
                    eval += Self::evaluate_window(&[&cells[i][j], 
                                                    &cells[i + 1][j - 1], 
                                                    &cells[i + 2][j - 2]]);
                }
            }
        }
        
        if n > 3 {
            let center = n / 2;
                for i in center.saturating_sub(1)..=(center + 1).min(n - 1) {
                    for j in center.saturating_sub(1)..=(center + 1).min(n - 1) {
                        match cells[i][j] {
                            Cell::X => eval += 5,
                            Cell::O => eval -= 5,
                            _ => {}
                        }
                    }
                }
            }
        eval
    }

    fn minimax(
        board: &mut Board,
        player: Player,
        depth: u32,
        max_depth: u32,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, usize, usize) {

        if depth == max_depth {
            let predicted_score = Self::heuristic(board);
            let depth_factor = (max_depth - depth) as i32;
            
            return (predicted_score + 
                if predicted_score > 0 {depth_factor} 
                else {-depth_factor}, 
                0, 0);
        }
        if board.game_over() {
            let score = board.score();
            let final_score = 
                if score > 0 {1000} 
                else if score < 0 {-1000} 
                else {0};
            let depth_penalty = depth as i32 * 100;
        
            return (
            if final_score > 0 {final_score - depth_penalty}
            else if final_score < 0 {final_score + depth_penalty}
            else {0},
            0, 0
        )};

        let mut moves = board.moves();

        if moves.is_empty() {
            return (0, 0, 0);
        }

        let cells = board.get_cells();
        let n = cells.len();
        moves.sort_by_cached_key(|&(x, y)| {
            let mut score = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        match cells[nx as usize][ny as usize] {
                            Cell::X if player == Player::X => score += 10,
                            Cell::O if player == Player::O => score += 10,
                            Cell::X | Cell::O => score += 5,
                            _ => {}
                        }
                    }
                }
            }
            -score
        });

        let maximizing = player == Player::X;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
        let mut best_move = (0, 0);
        let mut first = true;

        for (x, y) in moves {
            board.apply_move((x, y), player);
            
            let next_player = 
                if player == Player::X { Player::O } 
                else { Player::X };
            
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
                if first || score > best_score {
                    best_score = score;
                    best_move = (x, y);
                    first = false;
                }
                alpha = alpha.max(score);
            } 
            else {
                if first || score < best_score {
                    best_score = score;
                    best_move = (x, y);
                    first = false;
                }
                beta = beta.min(score);
            }

            if alpha >= beta {
                break; // Alpha-beta pruning
            }
        }

        return (best_score, best_move.0, best_move.1)
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let board_size = board.get_cells().len();

        // Deeper search for smaller boards
        let max_depth: u32 = match board_size {
            3 => 9,      // full solve for 3x3
            5 => 6,      // 5x5
            _ => 4,      // Limited for larger boards
        };

        Self::minimax(board, player, 0, max_depth, i32::MIN, i32::MAX)
    }
}