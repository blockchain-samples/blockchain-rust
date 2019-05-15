use crate::Block;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pendingTransactions: Vec<Transaction>,
    difficulty: usize
}

impl Blockchain{
    pub fn new() -> Blockchain{
        let mut bc = Blockchain{
            chain: vec![],
            pendingTransactions: vec![],
            difficulty: 2
        };
        bc.add_genesis_block();
        bc
    }

    fn add_genesis_block(&mut self) {
        let mut b = Block::new(None);
        b.set_previous_hash("".to_string());
        b.set_hash();

        self.chain.push(b);
    }

    pub fn add_block(&mut self, block: &mut Block) {
        let h = self.chain.last().expect("chain should not be empty");
        let hash = h.clone().hash;
        block.set_previous_hash(hash);
        block.mine(self.difficulty);
        self.chain.push(block.clone())
    }

    pub fn print(&self){
        println!("blockchain({}) [", self.chain.len());
        for b in self.chain.iter(){
            b.print();
        }
        println!("]");
    }

    pub fn is_valid(&self) -> bool{
        for i in 1..self.chain.len()-1 {
            let block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if block.hash != block.calc_hash() {
                return false;
            }

            if block.previous_hash != previous_block.hash {
                return false;
            }
        };
        true
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_blockchain() {
        let bc = Blockchain::new();
        let t = &bc.chain[0];

        assert!(match t.data {
            None => true,
            _ => false
        });
    }

    #[test]
    fn add_block() {
        let mut b1 = Block::new(Some(Transaction::new("a".to_string(), "b".to_string(), 1)));
        let mut b2 = Block::new(Some(Transaction::new("b".to_string(), "c".to_string(), 2)));
        let mut bc = Blockchain::new();
        assert_eq!(bc.chain.len(), 1);

        bc.add_block(&mut b1);
        assert_eq!(bc.chain.len(), 2);

        bc.add_block(&mut b2);
        assert_eq!(bc.chain.len(), 3);
    }

    #[test]
    fn validation() {
        let mut bc = Blockchain::new();
        assert!(bc.is_valid());

        let mut b1 = Block::new(Some(Transaction::new("a".to_string(), "b".to_string(), 1)));
        let mut b2 = Block::new(Some(Transaction::new("b".to_string(), "c".to_string(), 2)));
        bc.add_block(&mut b1);
        bc.add_block(&mut b2);
        assert!(bc.is_valid());

        //tempering with the chain data
        bc.chain[1].data = Some(Transaction::new("a".to_string(), "b".to_string(), 10));

        assert!(!bc.is_valid());
    }
}