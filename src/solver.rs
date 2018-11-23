use board::BoardGame;

pub fn solve(board_game: &mut BoardGame) {
    solveception(board_game, 0, 0);
}

pub fn solveception(board_game: &mut BoardGame, x: u8, y: u8) {
    let pieces = board_game.find_placable_pieces(x, y);

    for piece in pieces {
        board_game.cells[x as usize][y as usize].put(piece)
    }
}
