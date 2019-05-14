extern crate crypto;
extern crate base64;

use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;
use std::time::Instant;

#[derive(Clone)]
pub struct Block {
    pub data: u32,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
    pub time_mining: u128
}

impl Block {
    pub fn new(data: u32) -> Block{
        Block{
            data,
            previous_hash: "".to_string(),
            hash: "".to_string(),
            nonce: 0,
            time_mining: 0
        }
    }

    pub fn print(&self){
        println!("\tBlock{{\n\t\tdata: {}\n\t\thash: {}\n\t\tprevious_hash: {}\n\t\tnonce: {}\n\t\ttime_mining: {}ms\n\t}},",
                 self.data, &self.hash, &self.previous_hash, self.nonce, self.time_mining);
    }

    pub fn calc_hash(&self) -> String{
        let mut sha256 = Sha256::new();
        let hashable = format!("{}-{}-{}", self.data, self.previous_hash, self.nonce);
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

    pub fn mine(&mut self, difficulty: usize){
        let lead = "0".repeat(difficulty);
        let now = Instant::now();
        while self.hash == "" || self.hash[..difficulty] != lead {
            self.nonce+=1;
            self.set_hash();
        }
        self.time_mining = now.elapsed().as_millis();
        println!("time to mine: {}", now.elapsed().as_millis());
    }
}