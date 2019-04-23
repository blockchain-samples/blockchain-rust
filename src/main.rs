#[derive(Clone)]
struct Block {
    data: u32,
    previous_hash: String,
    hash: String
}

impl Block {
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
    fn add_block(&mut self, block: Block) {
        self.chain.push(block)
    }

    fn print(&self){
        for b in self.chain.iter(){
            println!("{}",b.data);
        }
    }
}

fn main() {
    let mut b = Block{
        data: 1,
        previous_hash: "".to_string(),
        hash: "".to_string()
    };

    let mut bc = Blockchain{ chain : Vec::new()};

    bc.add_block(b);

    bc.print();
}
