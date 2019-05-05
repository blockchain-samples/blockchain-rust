extern crate crypto;

use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;

#[derive(Clone)]
struct Block {
    data: u32,
    previous_hash: String,
    hash: String
}

impl Block {
    fn new(data: u32) -> Block{
        Block{
            data,
            previous_hash: "".to_string(),
            hash: "".to_string()
        }
    }

    fn to_string(&self) -> String {
        format!("data {}", self.data)
    }

    fn calc_hash(&mut self) {
        let mut hasher = Sha512::new();
        let hashable = self.previous_hash.to_string() + &self.data.to_string();
        hasher.input_str(&hashable);
        self.hash = hasher.result_str();
    }

    fn set_previous_hash(&mut self, hash: String){
        self.previous_hash = hash;
    }
}

#[derive(Clone)]
struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain{
    fn new() -> Blockchain{
        let mut b = Block::new(0);
        b.set_previous_hash("initialization block".to_string());
        b.calc_hash();

        Blockchain{
            chain: vec![b]
        }
    }

    fn add_block(&mut self, block: &mut Block) {
        let h = self.chain.last().expect("chain should not be empty");
        let hash = h.clone().hash;
        block.set_previous_hash(hash);
        block.calc_hash();
        self.chain.push(block.clone())
    }

    fn print(&self){
        println!("blockchain({})", self.chain.len());
        for b in self.chain.iter(){
            println!("{} - {} - {}",b.data, &b.hash[0..10], &b.previous_hash[0..10]);
        }
    }
}

fn main() {
    let mut b1 = Block::new(1);
    let mut b2 = Block::new(2);

    let mut bc = Blockchain::new();

    bc.print();

    bc.add_block(&mut b1);

    bc.print();

    bc.add_block(&mut b2);

    bc.print();
}
