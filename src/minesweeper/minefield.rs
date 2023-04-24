use super::block::Block;
use std::fmt;
use std::fmt::Debug;

type Result<T> = std::result::Result<T, MinefieldError>;
pub type Coordinate = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub enum MinefieldError {
    OutOfBound(Coordinate),
}

#[derive(Debug)]
pub struct Minefield {
    field: Vec<Vec<Block>>,
}

impl Minefield {
    pub fn new(length: usize, width: usize) -> Minefield {
        let mut field_builder: Vec<Vec<Block>> = Vec::with_capacity(length);

        for y in 0..length {
            field_builder.push(Vec::with_capacity(width));
            for _x in 0..width {
                field_builder[y].push(Block::new());
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

    fn get_adjacent_blocks_coord_at_coord(&self, coord: Coordinate) -> Result<Vec<Coordinate>> {
        // Checking out of bound access
        self.get_block_at_coord(coord)?;
        let mut adjacent_blocks_coord: Vec<Coordinate> = Vec::new();
        for i in 0..=2 {
            for j in 0..=2 {
                if (coord.0 != 0 || i != 0) && (coord.1 != 0 || j != 0) && (i != 1 || j != 1) {
                    let adj_block_coord: Coordinate = (coord.0 + i - 1, coord.1 + j - 1);
                    if self.get_block_at_coord(adj_block_coord).is_ok() {
                        adjacent_blocks_coord.push(adj_block_coord);
                    }
                }
            }
        }
        Ok(adjacent_blocks_coord)
    }

    pub fn get_nof_adjacent_mine_at_coord(&mut self, coord: Coordinate) -> Result<usize> {
        let adjacent_blocks_coord = self.get_adjacent_blocks_coord_at_coord(coord)?;
        let mut number_of_adjacent_mine = 0;
        for block_coord in adjacent_blocks_coord.iter() {
            if self.get_block_at_coord(*block_coord)?.is_mined() {
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
        let mut block: Block = Block::new();

        assert_eq!(
            empty_minefield.get_mut_block_at_coord((0, 0)).err(),
            Some(MinefieldError::OutOfBound((0, 0)))
        );

        assert_eq!(minefield.get_mut_block_at_coord((0, 0)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((2, 1)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((4, 0)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((3, 4)).is_ok(), true);
        assert_eq!(minefield.get_mut_block_at_coord((4, 4)).is_ok(), true);

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
        let mut block: Block = Block::new();

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
}
