use std::error::Error;
use std::fmt;
use std::fmt::Debug;

use rand::Rng;

use super::block::Coordinate;
use super::minefield::*;

type Result<T> = std::result::Result<T, MinesweeperGameError>;

#[derive(Debug)]
pub struct MinesweeperGame {
    pub minefield: Minefield,
}

impl MinesweeperGame {
    pub fn new(length: usize, width: usize) -> MinesweeperGame {
        MinesweeperGame {
            minefield: Minefield::new(length, width),
        }
    }

    pub fn populate_minefield(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..self.minefield.get_length() {
            for x in 0..self.minefield.get_width() {
                if rng.gen_range(0..100) < 15 {
                    match self.minefield.get_mut_block_at_coord((x, y)) {
                        Ok(block) => block.set_mined(true),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }

    pub fn sweep(&mut self, coord: Coordinate) -> Result<Vec<Coordinate>> {
        let mut swept_block = self.minefield.get_mut_block_at_coord(coord)?;
        if swept_block.is_swept() {
            Err(MinesweeperGameError::AlreadySwept(coord))
        } else {
            Ok(vec![])
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MinesweeperGameError {
    Minefield { source: MinefieldError },
    AlreadySwept(Coordinate),
}

impl fmt::Display for MinesweeperGameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MinesweeperGameError::Minefield { source } => {
                write!(f, "{}", source)
            }
            MinesweeperGameError::AlreadySwept(coord) => {
                write!(
                    f,
                    "Mine at coordinate ({}, {}) already swept",
                    coord.0, coord.1
                )
            }
        }
    }
}

impl Error for MinesweeperGameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MinesweeperGameError::Minefield { source } => Some(source),
            _ => None,
        }
    }
}

impl From<MinefieldError> for MinesweeperGameError {
    fn from(cause: MinefieldError) -> Self {
        MinesweeperGameError::Minefield { source: (cause) }
    }
}
