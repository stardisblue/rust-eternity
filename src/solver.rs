use board::BoardGame;
use crawler::Crawler;

pub fn solve(board_game: &mut BoardGame) {
    let crawler = Crawler::new(board_game.size);

    solveception(board_game, crawler);
}

pub fn solveception(board_game: &mut BoardGame, mut crawler: Crawler) {
    if let Some((x, y)) = crawler.current() {
        let pieces = board_game.find_placable_pieces(x, y);

        for piece in pieces {
            board_game.cells[x as usize][y as usize].put(piece);
        }
    }
}
