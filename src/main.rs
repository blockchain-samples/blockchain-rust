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
        self.hash = "1".to_string()
    }
}

#[derive(Clone)]
struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain{
    fn new() -> Blockchain{
        let mut b = Block::new(0);
        b.calc_hash();

        Blockchain{
            chain: vec![b]
        }
    }

    fn add_block(&mut self, block: Block) {

        self.chain.push(block)
    }

    fn print(&self){
        println!("blockchain({})", self.chain.len());
        for b in self.chain.iter(){
            println!("{} - {} - {}",b.data, b.hash, b.previous_hash);
        }
    }
}

fn main() {
    let mut b1 = Block::new(1);
    let mut b2 = Block::new(2);

    let mut bc = Blockchain::new();

    bc.print();

    bc.add_block(b1);

    bc.print();
}
