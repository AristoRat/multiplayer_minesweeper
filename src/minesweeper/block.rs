
#[derive(Debug)]
pub struct Block {
    x: usize,
    y: usize,
    nof_adjacent_mine: u8,
    mined: bool,
    flagged: bool,
    swept: bool,
}

impl Block {
    pub fn new(x: usize, y: usize) -> Block {
        Block {
            x,
            y,
            nof_adjacent_mine: 0,
            mined: false,
            flagged: false,
            swept: false,
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_nof_adjacent_mine(&self) -> u8 {
        self.nof_adjacent_mine
    }

    pub fn set_nof_adjacent_mine(&mut self, nof_adjacent_mine: u8) {
        self.nof_adjacent_mine = nof_adjacent_mine;
    }

    pub fn get_mined(&self) -> bool {
        self.mined
    }

    pub fn set_mined(&mut self, mined: bool) {
        self.mined = mined;
    }

    pub fn get_flagged(&self) -> bool {
        self.flagged
    }

    pub fn set_flagged(&mut self, flagged: bool) {
        self.flagged = flagged;
    }

    pub fn get_swept(&self) -> bool {
        self.swept
    }

    pub fn set_swept(&mut self, swept: bool) {
        self.swept = swept;
    }
}