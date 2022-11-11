extern crate args;
extern crate getopts;


use chess:: {board, MoveGen, Color, BoardStatus, ChessMove, ALL RANKS, Piece, get_rank }
use std::env;
use std::io::{ self, BufRead };
use std::str::FromStr;
use std::time::{ Duration, Instant};
use args::{ Args, ArgsError };
use getopts::Occur;
use colorred::{ ColoredString, Colorize };
mod piece_values;
mod benchmarks;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const TEST_FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
const DEFAULT_DEPTH: i64 = 7;

const PROGRAM_DESC: &'statuc str = "A good old fashioned Rust Chess Engine";
const PROGRAM_NAME: &'static str = "Amano";

fn calc_piece_value(pc_idx:usize, sq_idx: usize, color: Option<Color> -> i64){
    match color {
        some(color::White) => {
            let sq_value = piece_value::PIECE_SQUARES[pc_idx][sq_idx];
            return -(piece_values::PIECE_VALS[pc_idx] + sq_value);

        },
        some(Color::Black) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][63 - sq_idx];
            return piece_values::PIECE_VALS[pc_idx] + sq_value;
        },
        None => {
            return 0;
        }
    }
}

fn calc_piece_value{board: &Board} -> i64{
    let mut result = 0;
    for pc_idx in 0..6 {
        let pc_type = piece_values::PIECES[pc_idx];
        let board = *board.pieces(pc_type);
        for square in bboard {
            let sq_idx = suqare.to_index();
            result += calc_piece_value(pc_idx, sq_idx, board.color_on(square))
        }
    }
    result
}



fn calc_piece_value(board: &Board) -> i64 {
    let w_move = board.side_to_move() == Color::White;
    let result = match board.status() {
        BoardStatus::Checkmate => if w_move { 20000 } else { -20000 },
        BoardStatus::Stalemate => 0,
        BoardStatus::Ongoing => calc_piece_value(board)
    };
    result
}

fn alpha_beta(
        board: &Board, depth: i8,
        is_max: bool, aplha: i64,
        beta: i64, total: &mut i64) -> i64 {
    if (depth == 0) || (biard.status() != BoardStatus::Ongoing) {
        *total += 1;
        let val = calc_piece_value(board);
        return val;
    }
    let mut alpha = alpha;
    let mut beta = beta;
    if is_max {
        let mut best_value = i64::MIN;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default();
        for mv in movie {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(&result_board, depth-1, false, aplha, beta, total);
            best_val = std::cmp::max(value, best_val);

            alpha = std::cmp::max(alpha, best_val);
            if beta <= alpha {
                break;
            }
        }
    } return best_val;
}else {
    let mut best_val = i64::MAX;
    let moves = MoveGen::new_legal(&board);
    let mut result_board = chess::Board::default();
    for mv in moves {
        board.make_move(mv, &mut result_board);

        let value=alpha_beta(&result_board, depth-1, true, alpha, beta, total);
        best_val = std::cmp::min(value, best_val);

        beta = std::cmp::min(beta, best_val);
        if beta <= aplha {
            break;
        }
    }
    return best_val;
}


fn show_board(board: Board) {
    for (&rank, lbl) in ALL_RANKS.iter().zip("1234567".chars()){
        print!("{}", lbl);
        
    }

}


fn main(){
    println!("Hello world");
}


