mod block;
mod blockchain;
mod transaction;

use block::Block;
use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut b1 = Block::new(Some(Transaction::new("a".to_string(), "b".to_string(), 1)));
    let mut b2 = Block::new(Some(Transaction::new("b".to_string(), "c".to_string(), 2)));
    let mut bc = Blockchain::new();

    bc.print();

    bc.add_block(&mut b1);

    bc.print();

    bc.add_block(&mut b2);

    bc.print();

    println!("{}", bc.is_valid());

    bc.chain[1].data = Some(Transaction::new("a".to_string(), "b".to_string(), 10));

    println!("{}", bc.is_valid());
}
