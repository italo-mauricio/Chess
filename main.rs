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

fn main(){
    println!("Hello world");
}