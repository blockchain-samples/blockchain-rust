use std::fmt;

#[derive(Clone)]
pub struct Transaction{
    pub from: String,
    pub to: String,
    pub coins: u32
}

impl Transaction{
    pub fn new(from: String, to: String, coins: u32) -> Transaction{
        Transaction{
            from, to, coins
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Transaction: {})", self.coins)
    }
}