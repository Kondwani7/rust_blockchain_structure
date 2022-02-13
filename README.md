# rust_blockchain_structure

This demonstrates how a blockchain is created and expands in rust based on the following steps:

clone the repo and run:
### `cargo run`
or
### `cargo build`
In your terminal you will be presented with 4 transactions:
```bash
input miner_address: "any name of your choice"
difficulty: (betweeen 1-4 to prevent a strain in your RAM)
```
After which a new block will be created similar to the example below
```bash
lock hash: 0005ce62391ffaeb4355fdacb5885f509774aec3dbc1378e6e1a770c2
Block {
    header: BlockHeader {
        timestamp: 1644780516093,
        nonce: 3290674,
        prev_hash: "00d690d6b947a5d5a387445e95f24b9df47f5edfe96a25bb04687c7159c",
        merkle: "8db824c1d11327ad1b4c3d8b2f89acb5df6c888e206cc42793806ff4ccc675c",
        difficulty: 3,
    },
    count: 1,
    transactions: [
        Transaction {
            sender: "Root",
            receiver: "kdee",
            amount: 213.5,
        },
    ],
}
```

you will be presented with 4 options, namely: 
```bash
1)New Transaction
2)Mine block
3)Update difficulty
4)Update reward
5) Exit
```
)ption 1 allows your to send some amount to a new reciever of your choice.
After entering the desired sender receiver addresses, and the amount,

option 2 (Mine block) to add the new transaction to the blockchain

Option 3 allows you to change the difficulty to create new block and reward proof of work. it increases the nonce value

Option 4 increases the reward assigned to each sender when a new block is created. after assigning a new reward, e.g:
### `Enter new reward: 213.5`
 
and option 2 to generate a new block.

The new block will be created with inital amount assigned changed

Any option greater than 4 will return an invalid entry:
### `Invalid option please retry`

