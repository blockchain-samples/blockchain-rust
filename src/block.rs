extern crate crypto;
extern crate base64;

use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;

#[derive(Clone)]
pub struct Block {
    pub data: u32,
    pub previous_hash: String,
    pub hash: String
}

impl Block {
    pub fn new(data: u32) -> Block{
        Block{
            data,
            previous_hash: "".to_string(),
            hash: "".to_string()
        }
    }

    pub fn print(&self){
        println!("\tBlock{{\n\t\tdata: {}\n\t\thash: {}\n\t\tprevious_hash: {}\n\t}},",self.data, base64::encode(&self.hash), base64::encode(&self.previous_hash));
    }

    pub fn calc_hash(&mut self) -> String{
        let mut sha256 = Sha256::new();
        let hashable = format!("{}-{}", self.data, self.previous_hash);
        sha256.input_str(&hashable);
        let output = sha256.result_str();
        return output;
    }

    pub fn set_previous_hash(&mut self, hash: String){
        self.previous_hash = hash;
    }

    pub fn set_hash(&mut self) {
        self.hash = self.calc_hash();
    }
}