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
        /*for (int i = 0; i < config.getMineProportion() / 100. * size; i++) {

            int x;
            int y;
            do {
                x = random.nextInt(width);
                y = random.nextInt(height);
            } while (board[x][y].getValue() == mineValue);
            board[x][y].setValue(mineValue);

            //update of the adjacent mine's square with their value
            int I = x, J = y; //current position
            for (int dirX = -1; dirX <= 1; ++dirX) {
                for (int dirY = -1; dirY <= 1; ++dirY) {
                    //exclude the case (0,0)
                    if (dirX != 0 || dirY != 0) {
                        if (I + dirX >= 0 && I + dirX < width &&
                            J + dirY >= 0 && J + dirY < height) {

                            int value = board[I + dirX][J + dirY].getValue();
                            if (value != mineValue)
                            board[I + dirX][J + dirY].setValue(value + 1);
                        }
                    }
                }
            }
        }*/
    }
}