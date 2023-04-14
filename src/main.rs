mod minesweeper;

use crate::minesweeper::block::Block;

fn main() {
    let block: Block = Block::new(1,2);
    println!("{:?}", block.get_x());
}
