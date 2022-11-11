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
fn main(){
    println!("Hello world");
}


