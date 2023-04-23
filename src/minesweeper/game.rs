use rand::Rng;

use super::minefield::Minefield;

#[derive(Debug)]
pub struct MinesweeperGame {
    pub minefield: Minefield,
    is_game_finished: bool,
}

impl MinesweeperGame {
    pub fn new(length: usize, width: usize) -> MinesweeperGame {
        MinesweeperGame {
            minefield: Minefield::new(length, width),
            is_game_finished: false,
        }
    }

    pub fn populate_minefield(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.minefield.get_length() {
            for x in 0..self.minefield.get_width() {
                if rng.gen_range(0..100) < 15 {
                    match self.minefield.inc_adjacent_block_value(x, y) {
                        Ok(_) => match self.minefield.get_mut_block_at_coord(x, y) {
                            Ok(block) => block.set_mined(true),
                            Err(e) => println!("{}", e),
                        },
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }
}