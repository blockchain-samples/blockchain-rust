mod block;
mod blockchain;

use block::Block;
use blockchain::Blockchain;

fn main() {
    let mut b1 = Block::new(1);
    let mut b2 = Block::new(2);
    let mut bc = Blockchain::new();

    bc.print();

    bc.add_block(&mut b1);

    bc.print();

    bc.add_block(&mut b2);

    bc.print();

    println!("{}", bc.is_valid());

    bc.chain[1].data = 10;

    println!("{}", bc.is_valid());
}
