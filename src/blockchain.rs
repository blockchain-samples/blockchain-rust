use crate::Block;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    difficulty: usize
}

impl Blockchain{
    pub fn new() -> Blockchain{
        let mut bc = Blockchain{
            chain: vec![],
            pending_transactions: vec![],
            difficulty: 2
        };
        bc.add_genesis_block();
        bc
    }

    fn add_genesis_block(&mut self) {
        let mut b = Block::new(vec![]);
        b.set_previous_hash("".to_string());
        b.mine(self.difficulty);

        self.chain.push(b);
    }

    fn add_block(&mut self, block: &mut Block) {
        let h = self.chain.last().expect("chain should not be empty");
        let hash = h.clone().hash;
        block.set_previous_hash(hash);
        block.mine(self.difficulty);
        self.chain.push(block.clone())
    }

    pub fn generate_block(&mut self) {
        let mut block = Block::new(self.pending_transactions.clone());
        self.add_block(&mut block);
    }

    pub fn print(&self){
        println!("blockchain({}) [", self.chain.len());
        for b in self.chain.iter(){
            b.print();
        }
        println!("]");
    }

    pub fn is_valid(&self) -> bool{
        for i in 1..self.chain.len() {
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

    pub fn add_to_pending(&mut self, transaction: Transaction){
        self.pending_transactions.push(transaction);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_blockchain() {
        let bc = Blockchain::new();
        let genesis = &bc.chain[0];

        assert_eq!(genesis.data.len(), 0);
        assert_eq!(genesis.hash, genesis.calc_hash());
    }

    #[test]
    fn add_block() {
        let mut bc = Blockchain::new();
        assert_eq!(bc.chain.len(), 1);

        bc.add_to_pending(Transaction::new("a".to_string(), "b".to_string(), 1));
        bc.add_to_pending(Transaction::new("c".to_string(), "d".to_string(), 2));
        bc.generate_block();
        assert_eq!(bc.chain.len(), 2);
    }

    #[test]
    fn validation() {
        let mut bc = Blockchain::new();
        assert!(bc.is_valid());

        bc.add_to_pending(Transaction::new("a".to_string(), "b".to_string(), 1));
        bc.add_to_pending(Transaction::new("c".to_string(), "d".to_string(), 2));
        bc.generate_block();
        assert!(bc.is_valid());

        //tempering with the chain data
        bc.chain[1].data[0].coins = 100;

        assert!(!bc.is_valid());
    }
}