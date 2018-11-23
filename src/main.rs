#[allow(unused)]
mod board;
mod solver;

use board::BoardGame;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut board_game = BoardGame::new(contents);
    println!("{:#?}", board_game.pieces[4]);

    board_game
        .pieces
        .get_mut(4)
        .expect("out of bounds")
        .rotate(board::Rotate::Half);
    println!("{:#?}", board_game.pieces[4]);

    board_game.cells[2][2].put(board_game.pieces[4].id);
    println!("{:#?}", board_game);
}
