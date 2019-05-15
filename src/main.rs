mod block;
mod blockchain;
mod transaction;

use block::Block;
use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut bc = Blockchain::new();

    bc.add_to_pending(Transaction::new("a".to_string(), "b".to_string(), 1));
    bc.add_to_pending(Transaction::new("c".to_string(), "d".to_string(), 2));
    bc.generate_block();

    bc.print();
    println!("{}", bc.is_valid());
    println!("{}", bc.chain[1].calc_hash());

    bc.chain[1].data[0].coins = 100;
    println!("{}", bc.chain[1].calc_hash());

    bc.print();
    println!("{}", bc.is_valid());
}
