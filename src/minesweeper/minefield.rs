use std::error::Error;
use std::fmt;
use std::fmt::Debug;

use super::block::{Block, Coordinate};

#[derive(Debug)]
pub struct Minefield {
    field: Vec<Vec<Block>>,
}

impl Minefield {
    pub fn new(length: usize, width: usize) -> Minefield {
        let mut field_builder: Vec<Vec<Block>> = Vec::with_capacity(length);

        for y in 0..length {
            field_builder.push(Vec::with_capacity(width));
            for x in 0..width {
                field_builder[y].push(Block::new((x, y)));
            }
        }

        // TODO: throw error if length or width are smaller than 0.
        Minefield {
            field: field_builder,
        }
    }

    pub fn get_length(&self) -> usize {
        self.field.len()
    }

    pub fn get_width(&self) -> usize {
        if self.field.is_empty() {
            0
        } else {
            self.field.get(0).unwrap().len()
        }
    }

    pub fn get_mut_block_at_coord(&mut self, coord: Coordinate) -> Result<&mut Block> {
        let err = MinefieldError::OutOfBound(coord);
        self.field
            .get_mut(coord.1)
            .ok_or(err.clone())?
            .get_mut(coord.0)
            .ok_or(err)
    }

    pub fn get_block_at_coord(&self, coord: Coordinate) -> Result<&Block> {
        let err = MinefieldError::OutOfBound(coord);
        self.field
            .get(coord.1)
            .ok_or(err.clone())?
            .get(coord.0)
            .ok_or(err)
    }

    fn get_adjacent_blocks_at_coord(&self, coord: Coordinate) -> Result<Vec<&Block>> {
        // Checking out of bound access
        self.get_block_at_coord(coord)?;
        let mut adjacent_blocks: Vec<&Block> = Vec::new();
        for i in 0..=2 {
            for j in 0..=2 {
                if (coord.0 != 0 || i != 0) && (coord.1 != 0 || j != 0) && (i != 1 || j != 1) {
                    let adj_block_coord: Coordinate = (coord.0 + i - 1, coord.1 + j - 1);
                    if let Ok(adj_block) = self.get_block_at_coord(adj_block_coord) {
                        adjacent_blocks.push(adj_block);
                    }
                }
            }
        }
        Ok(adjacent_blocks)
    }

    /*fn get_adjacent_mut_blocks_at_coord(&mut self, coord: Coordinate) -> Result<Vec<&mut Block>> {
        // Checking out of bound access
        self.get_block_at_coord(coord)?;
        let mut adjacent_blocks: Vec<&mut Block> = Vec::new();
        for i in 0..=2 {
            for j in 0..=2 {
                if (coord.0 != 0 || i != 0) && (coord.1 != 0 || j != 0) && (i != 1 || j != 1) {
                    let adj_block_coord: Coordinate = (coord.0 + i - 1, coord.1 + j - 1);
                    if let Ok(adj_block) = self.get_mut_block_at_coord(adj_block_coord) {
                        adjacent_blocks.push(adj_block);
                    }
                }
            }
        }
        Ok(adjacent_blocks)
    }*/

    pub fn test_tmp(&mut self, coord: Coordinate) -> Result<()> {
        let adjacent_blocks = self.get_adjacent_blocks_at_coord(coord)?;
        let mut number_of_adjacent_mine = 0;
        for block in adjacent_blocks.iter() {
            self.get_mut_block_at_coord(block.get_coordinate()).unwrap().set_mined(true);
        }
        Ok(())
    }

    pub fn get_nof_adjacent_mine_at_coord(&self, coord: Coordinate) -> Result<usize> {
        let adjacent_blocks = self.get_adjacent_blocks_at_coord(coord)?;
        let mut number_of_adjacent_mine = 0;
        for block in adjacent_blocks.iter() {
            if block.is_mined() {
                number_of_adjacent_mine += 1;
            }
        }
        Ok(number_of_adjacent_mine)
    }
}

impl fmt::Display for Minefield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                write!(
                    f,
                    " {}",
                    if self.field[y][x].is_flagged() {
                        "?".to_string()
                    } else if self.field[y][x].is_swept() {
                        "X".to_string()
                    } else if self.field[y][x].is_mined() {
                        "M".to_string()
                    } else {
                        self.field[y][x].get_nof_adjacent_mine().to_string()
                    }
                )?;
            }
            if y + 1 != self.field.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

type Result<T> = std::result::Result<T, MinefieldError>;

#[derive(Debug, Clone, PartialEq)]
pub enum MinefieldError {
    OutOfBound(Coordinate),
}

impl fmt::Display for MinefieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinefieldError::OutOfBound(coord) => {
                write!(
                    f,
                    "Cannot access block at coordinate ({}, {})",
                    coord.0, coord.1
                )
            }
        }
    }
}

impl Error for MinefieldError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_length() {
        let empty_minefield1: Minefield = Minefield { field: vec![] };
        let empty_minefield2: Minefield = Minefield {
            field: vec![vec![]],
        };
        let minefield1: Minefield = Minefield::new(3, 3);
        let minefield2: Minefield = Minefield::new(0, 10);
        let minefield3: Minefield = Minefield::new(25, 40);

        assert_eq!(empty_minefield1.get_length(), 0);
        assert_eq!(empty_minefield2.get_length(), 1);

        assert_eq!(minefield1.get_length(), 3);
        assert_eq!(minefield2.get_length(), 0);
        assert_eq!(minefield3.get_length(), 25);
        assert_ne!(minefield3.get_length(), 40);
    }

    #[test]
    fn get_width() {
        let empty_minefield1: Minefield = Minefield { field: vec![] };
        let empty_minefield2: Minefield = Minefield {
            field: vec![vec![]],
        };
        let minefield1: Minefield = Minefield::new(3, 3);
        let minefield2: Minefield = Minefield::new(0, 10);
        let minefield3: Minefield = Minefield::new(25, 40);

        assert_eq!(empty_minefield1.get_width(), 0);
        assert_eq!(empty_minefield2.get_width(), 0);

        assert_eq!(minefield1.get_width(), 3);
        assert_eq!(minefield2.get_width(), 0);
        assert_ne!(minefield2.get_width(), 10);
        assert_eq!(minefield3.get_width(), 40);
        assert_ne!(minefield3.get_width(), 25);
    }

    #[test]
    fn test_get_mut_block_at_coord() {
        let mut empty_minefield: Minefield = Minefield { field: vec![] };
        let mut minefield: Minefield = Minefield::new(5, 5);

        assert_eq!(
            empty_minefield.get_mut_block_at_coord((0, 0)).err(),
            Some(MinefieldError::OutOfBound((0, 0)))
        );

        assert_eq!(minefield.get_mut_block_at_coord((0, 0)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((2, 1)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((4, 0)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((3, 4)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((4, 4)).is_ok(), true);

        let mut block: Block = Block::new((4, 4));
        assert_eq!(minefield.get_mut_block_at_coord((4, 4)).unwrap(), &block);
        block.incr_nof_adjacent_mine();
        assert_ne!(minefield.get_mut_block_at_coord((4, 4)).unwrap(), &block);
        minefield
            .get_mut_block_at_coord((4, 4))
            .unwrap()
            .incr_nof_adjacent_mine();
        assert_eq!(minefield.get_mut_block_at_coord((4, 4)).unwrap(), &block);

        assert_eq!(
            minefield.get_mut_block_at_coord((5, 4)).err(),
            Some(MinefieldError::OutOfBound((5, 4)))
        );
        assert_eq!(
            minefield.get_mut_block_at_coord((4, 5)).err(),
            Some(MinefieldError::OutOfBound((4, 5)))
        );
        assert_eq!(
            minefield.get_mut_block_at_coord((5, 5)).err(),
            Some(MinefieldError::OutOfBound((5, 5)))
        );
        assert_ne!(
            minefield.get_mut_block_at_coord((7, 42)).err(),
            Some(MinefieldError::OutOfBound((42, 7)))
        );
        assert_eq!(
            minefield.get_mut_block_at_coord((1048041, 44198011)).err(),
            Some(MinefieldError::OutOfBound((1048041, 44198011)))
        );
    }

    #[test]
    fn test_get_block_at_coord() {
        let empty_minefield: Minefield = Minefield { field: vec![] };
        let minefield: Minefield = Minefield::new(5, 5);

        assert_eq!(
            empty_minefield.get_block_at_coord((0, 0)).err(),
            Some(MinefieldError::OutOfBound((0, 0)))
        );

        assert_eq!(minefield.get_block_at_coord((0, 0)).is_ok(), true);
        assert_eq!(minefield.get_block_at_coord((0, 0)).is_ok(), true);
        assert_eq!(minefield.get_block_at_coord((0, 1)).is_ok(), true);
        assert_eq!(minefield.get_block_at_coord((1, 2)).is_ok(), true);
        assert_eq!(minefield.get_block_at_coord((2, 3)).is_ok(), true);
        assert_eq!(minefield.get_block_at_coord((4, 4)).is_ok(), true);

        let mut block: Block = Block::new((4, 4));
        assert_eq!(minefield.get_block_at_coord((4, 4)).unwrap(), &block);
        block.incr_nof_adjacent_mine();
        assert_ne!(minefield.get_block_at_coord((4, 4)).unwrap(), &block);

        assert_eq!(
            minefield.get_block_at_coord((5, 4)).err(),
            Some(MinefieldError::OutOfBound((5, 4)))
        );
        assert_eq!(
            minefield.get_block_at_coord((4, 5)).err(),
            Some(MinefieldError::OutOfBound((4, 5)))
        );
        assert_eq!(
            minefield.get_block_at_coord((5, 5)).err(),
            Some(MinefieldError::OutOfBound((5, 5)))
        );
        assert_ne!(
            minefield.get_block_at_coord((7, 42)).err(),
            Some(MinefieldError::OutOfBound((42, 7)))
        );
        assert_eq!(
            minefield.get_block_at_coord((59187912, 9086109710)).err(),
            Some(MinefieldError::OutOfBound((59187912, 9086109710)))
        );
    }

    #[test]
    fn test_get_adjacent_blocks_at_coord() {
        let minefield: Minefield = Minefield::new(3, 3);
        assert_eq!(
            minefield.get_adjacent_blocks_at_coord((3, 0)).err(),
            Some(MinefieldError::OutOfBound((3, 0)))
        );

        assert_eq!(
            minefield.get_adjacent_blocks_at_coord((0, 0)).is_ok(),
            true
        );
        assert_eq!(
            minefield.get_adjacent_blocks_at_coord((2, 0)).is_ok(),
            true
        );
        assert_eq!(
            minefield.get_adjacent_blocks_at_coord((2, 2)).is_ok(),
            true
        );
        assert_eq!(
            minefield.get_adjacent_blocks_at_coord((1, 1)).is_ok(),
            true
        );
        assert_eq!(
            minefield.get_adjacent_blocks_at_coord((1, 0)).is_ok(),
            true
        );

        let mut adj_blocks = minefield
            .get_adjacent_blocks_at_coord((0, 0))
            .unwrap();
        assert_eq!(adj_blocks.len(), 3);
        assert_eq!(adj_blocks, vec![Block::new((0, 1)), Block::new((1, 0)), Block::new((1, 1))]);

        adj_blocks = minefield
            .get_adjacent_blocks_at_coord((1, 0))
            .unwrap();
        assert_eq!(adj_blocks.len(), 5);
        assert_eq!(
            adj_blocks,
            vec![Block::new((0, 0)), Block::new((0, 1)), Block::new((1, 1)), Block::new((2, 0)), Block::new((2, 1))]
        );

        adj_blocks = minefield
            .get_adjacent_blocks_at_coord((1, 1))
            .unwrap();
        assert_eq!(adj_blocks.len(), 8);
        assert_eq!(
            adj_blocks,
            vec![
                Block::new((0, 0)),
                Block::new((0, 1)),
                Block::new((0, 2)),
                Block::new((1, 0)),
                Block::new((1, 2)),
                Block::new((2, 0)),
                Block::new((2, 1)),
                Block::new((2, 2))
            ]
        );

        adj_blocks = minefield
            .get_adjacent_blocks_at_coord((2, 2))
            .unwrap();
        assert_eq!(adj_blocks.len(), 3);
        assert_eq!(adj_blocks, vec![Block::new((1, 1)), Block::new((1, 2)), Block::new((2, 1))]);
    }

    #[test]
    fn test_get_nof_adjacent_mine_at_coord() {
        let mut minefield = Minefield::new(8, 8);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((5, 8)).err(),
            Some(MinefieldError::OutOfBound((5, 8)))
        );

        minefield
            .get_mut_block_at_coord((0, 0))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((0, 1))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((0, 2))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((1, 0))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((1, 2))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((2, 0))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((2, 1))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((2, 2))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((4, 0))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((4, 5))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((4, 6))
            .unwrap()
            .set_mined(true);
        minefield
            .get_mut_block_at_coord((7, 3))
            .unwrap()
            .set_mined(true);
        println!("{}", minefield);

        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((1, 1)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((1, 1)).unwrap(), 8);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((6, 6)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((6, 6)).unwrap(), 0);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((7, 7)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((7, 7)).unwrap(), 0);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((6, 7)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((6, 7)).unwrap(), 0);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((3, 0)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((3, 0)).unwrap(), 3);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((3, 1)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((3, 1)).unwrap(), 4);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((3, 2)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((3, 2)).unwrap(), 2);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((1, 4)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((1, 3)).unwrap(), 3);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((3, 3)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((3, 4)).unwrap(), 1);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((3, 5)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((3, 5)).unwrap(), 2);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((0, 1)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((0, 1)).unwrap(), 4);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((1, 0)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((1, 0)).unwrap(), 4);
        assert_eq!(
            minefield.get_nof_adjacent_mine_at_coord((0, 0)).is_ok(),
            true
        );
        assert_eq!(minefield.get_nof_adjacent_mine_at_coord((0, 0)).unwrap(), 2);
    }
}
