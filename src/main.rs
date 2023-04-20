mod minesweeper;

use minesweeper::game::MinesweeperGame;

fn main() {
    let mut minesweeper_game: MinesweeperGame = MinesweeperGame::new(10, 20);
    minesweeper_game.populate_minefield();
    println!("{}", minesweeper_game.minefield);
    /*minesweeper_game.minefield.get_mut_block_at_coord(0, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(1, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(2, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(3, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(4, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(5, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(6, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(7, 5).unwrap().set_swept(true);
    minesweeper_game.minefield.get_mut_block_at_coord(8, 5).unwrap().set_swept(true);*/
    //println!("{}", minesweeper_game.minefield);
}
