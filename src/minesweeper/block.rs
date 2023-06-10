pub type Coordinate = (usize, usize);

#[derive(Debug, PartialEq)]
pub struct Block {
    coordinate: Coordinate,
    nof_adjacent_mine: usize,
    mined: bool,
    flagged: bool,
    swept: bool,
}

impl Block {
    pub fn new(coordinate: Coordinate) -> Block {
        Block {
            coordinate,
            nof_adjacent_mine: 0,
            mined: false,
            flagged: false,
            swept: false,
        }
    }

    /*pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }*/

    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate
    }

    pub fn get_nof_adjacent_mine(&self) -> usize {
        self.nof_adjacent_mine
    }

    pub fn set_nof_adjacent_mine(&mut self, nof_adjacent_mine: usize) {
        self.nof_adjacent_mine = nof_adjacent_mine;
    }

    pub fn incr_nof_adjacent_mine(&mut self) {
        self.nof_adjacent_mine += 1;
    }

    pub fn is_mined(&self) -> bool {
        self.mined
    }

    pub fn set_mined(&mut self, mined: bool) {
        self.mined = mined;
    }

    pub fn is_flagged(&self) -> bool {
        self.flagged
    }

    pub fn set_flagged(&mut self, flagged: bool) {
        self.flagged = flagged;
    }

    pub fn is_swept(&self) -> bool {
        self.swept
    }

    pub fn set_swept(&mut self, swept: bool) {
        self.swept = swept;
    }
}
