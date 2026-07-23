use crate::{
    board::{Board, State, Value},
    moves::Move,
};

// Minimax algorithm to determine the heuristic score of a given board state.
// - `depth`: The current depth of recursion, used to penalize longer paths to a win or reward longer paths to a loss.
// - `whos_turn`: The player (X or O) whose turn it is to move next.
// - `for_who`: The player (X or O) from whose perspective we are maximizing the score.
pub fn minimax(board: Board, depth: i32, whos_turn: Value, for_who: Value) -> i32 {
    // Base cases: if the game has ended, return the score modified by depth to prefer quicker wins/slower losses.
    match board.evaluate_board() {
        State::OnGoing => (),
        State::Draw => {
            return 0 - depth;
        }
        State::XWins => {
            if for_who == Value::X {
                return 10 - depth;
            }
            return -10 + depth;
        }
        State::OWins => {
            if for_who == Value::O {
                return 10 - depth;
            }
            return -10 + depth;
        }
    }

    // Determine if we are maximizing or minimizing the score.
    let is_maximizing = whos_turn == for_who;
    let mut m_score = if is_maximizing { i32::MIN } else { i32::MAX };

    // Explore all empty tiles and recurse.
    match whos_turn {
        Value::X => {
            for tile in 0..9 {
                if board.get_grid()[tile] != Value::None {
                    continue;
                }

                let mut board = board.clone();
                let move_v = Move::from(tile);

                board.place_x(move_v);

                let score = minimax(board, depth + 1, Value::O, for_who);

                if is_maximizing {
                    m_score = m_score.max(score)
                } else {
                    m_score = m_score.min(score);
                }
            }
        }
        Value::O => {
            for tile in 0..9 {
                if board.get_grid()[tile] != Value::None {
                    continue;
                }

                let mut board = board.clone();
                let move_v = Move::from(tile);

                board.place_o(move_v);

                let score = minimax(board, depth + 1, Value::X, for_who);

                if is_maximizing {
                    m_score = m_score.max(score);
                } else {
                    m_score = m_score.min(score);
                }
            }
        }
        _ => unreachable!(),
    }

    m_score
}

// Finds the optimal Move for the bot player using the minimax function.
pub fn find_best_move(board: &Board, bot: Value) -> Option<Move> {
    let mut best_score = i32::MIN;
    let mut best_tile = None;

    // Evaluate all possible moves for the bot and select the one with the highest minimax score.
    for tile in 0..9 {
        if board.get_grid()[tile] != Value::None {
            continue;
        }

        let mut board_clone = board.clone();
        let move_v = Move::from(tile);

        if bot == Value::X {
            board_clone.place_x(move_v);
        } else {
            board_clone.place_o(move_v);
        }

        let opponent = if bot == Value::X { Value::O } else { Value::X };

        // Minimax simulation starting at depth 0
        let score = minimax(board_clone, 0, opponent, bot);

        if score > best_score {
            best_score = score;
            best_tile = Some(move_v);
        }
    }

    best_tile
}
