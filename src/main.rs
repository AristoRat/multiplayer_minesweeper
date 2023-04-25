mod minesweeper;

use minesweeper::game::MinesweeperGame;

fn main() {
    let mut minesweeper_game: MinesweeperGame = MinesweeperGame::new(10, 20);
    minesweeper_game.populate_minefield();
    println!("{}", minesweeper_game.minefield);
    minesweeper_game.sweep((3, 3)).unwrap();
}
