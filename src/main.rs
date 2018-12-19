#[allow(unused)]
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod board;
mod cell;
mod piece;

use board::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let file = File::open(filename).expect("file not found");

    let mut contents = Vec::new();

    for line in BufReader::new(file).lines() {
        contents.push(line.expect("could not read line"))
    }

    let mut board_game = BoardGame::new(contents);
    board_game.put_piece(12, (1, 1), Some(Compass::East));
    println!("{:#?}", board_game.cells[1][1]);
    board_game.put_piece(3, (0, 0), None);
    println!("{:#?}", board_game.cells[0][0]);

    board_game.remove_piece((1, 1));
    println!("{:#?}", board_game.cells[1][1]);
}
