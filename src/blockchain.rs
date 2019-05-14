use crate::Block;

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    difficulty: usize
}

impl Blockchain{
    pub fn new() -> Blockchain{
        let mut bc = Blockchain{
            chain: vec![],
            difficulty: 2
        };
        bc.add_genesis_block();
        bc
    }

    fn add_genesis_block(&mut self) {
        let mut b = Block::new(0);
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