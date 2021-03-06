mod blockchain;

use std::io;
use std::io::Write;
use std::process;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();
    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating the genesis(first) block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change difficulty");
        println!("4) change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("enter reciever address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(), 
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res{
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }
            2 => {
                println!("Generating a new block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res{
                    true => println!("Updated Difficulty"),
                    false => println!("Failed to Update Difficulty")
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated Reward"),
                    false =>println!("Failed to Update Reward")
                }
            }
            //if user enters a number out of range 
            _=> println!("Invalid option please retry"),
        }
    }

}
