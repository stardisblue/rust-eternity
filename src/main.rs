#[allow(unused)]
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod board;
mod piece;

use board::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // --snip--
    println!("In file {}", filename);

    let file = File::open(filename).expect("file not found");

    let mut contents = Vec::new();

    for line in BufReader::new(file).lines() {
        contents.push(line.expect("could not read line"))
    }

    let board_game = BoardGame::new(contents);
    println!("{:#?}", board_game);
}
