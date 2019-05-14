use crate::Block;

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain{
    pub fn new() -> Blockchain{
        let mut b = Block::new(0);
        b.set_previous_hash("".to_string());
        b.set_hash();

        Blockchain{
            chain: vec![b]
        }
    }

    pub fn add_block(&mut self, block: &mut Block) {
        let h = self.chain.last().expect("chain should not be empty");
        let hash = h.clone().hash;
        block.set_previous_hash(hash);
        block.set_hash();
        self.chain.push(block.clone())
    }

    pub fn print(&self){
        println!("blockchain({}) [", self.chain.len());
        for b in self.chain.iter(){
            b.print();
        }
        println!("]");
    }
}