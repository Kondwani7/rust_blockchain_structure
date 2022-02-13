use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction{
    sender: String,
    receiver: String,
    amount: f32
}

#[derive(Debug, Serialize)]
pub struct BlockHeader{
    timestamp: i64,
    //number only used once for cryptography and generate a unqiue hash in a sequence
    nonce: u32,
    prev_hash: String,
    // it a tree data structure in a blockchain composed of hashes summmarizing all transactions in a block
    //it is the store of the digital footprint of blockchain transactions
    merkle: String,
    //used to adjust how hard it is for miner working to resolve a block
    difficulty: u32
}
#[derive(Debug, Serialize)]
pub struct Block{
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}

pub struct Chain{
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}
//implementing the blockchain
impl Chain{
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }
    //create a new transaction
    pub fn new_transaction(&mut self, sender:String, receiver: String, amount:f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }
    //creating the last chain of our blockchain which in the beginning doesnt exist
    pub fn last_hash(&self) -> String{
        let block = match self.chain.last() {
            Some(block) => block,
            //filling it with zeros intially if some block is not found
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }
    //update difficulty
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }
    //update reward
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }
    //generate new block
    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            prev_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };
        //rewarding the transaction from the new block created
        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };
        //creating the new block 
        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };
        //pushing elements to the block and proof of work to add to the Chain(Blockchain)
        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);
        //pushing the block to the blockchain
        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }
    //add a node to the a merkle tree based on the current transaction and hash
    fn get_merkle(curr_trans: Vec<Transaction>) -> String{
        let mut merkle = Vec::new();
        //generate a hash after iterating through a transaction
        for t in &curr_trans{
            let hash = Chain::hash(t);
            //push the hashes into the merkle vector
            merkle.push(hash);
        }
        //if the length of the merkle is odd push the last vector to the merkle list of vectors
        if merkle.len() %2 == 1{
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1{
            //first and second hash
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            //combine hash 1 with hash 2
            h1.push_str(&mut h2);
            //t hen create a new hash based  on the combo of hash 1 and hash 2
            let nh = Chain::hash(&h1);
            //after passing the new hash in the chain push it to the merkle
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }
//cryptographic proof that effort has been expended to create a new block &mint a token/currency
//helps deal with denial of service attacks
    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                //similar to a try and catch statment when making api calls
                Ok(val) => {
                    if val !=0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    pub fn hash<T : serde::Serialize>(item: &T) -> String{
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        //convert  the json to bytes then put in the the hasher
        hasher.update(input.as_bytes());
        //get the result made of bytes the convert it to a vector
        let res = hasher.finalize();
        let vec_res = res.to_vec();
        //converts vector to a string then slices it
        Chain:: hex_to_string(vec_res.as_slice())
    }
    //vector to string
    pub fn hex_to_string(vec_res: &[u8]) -> String{
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}