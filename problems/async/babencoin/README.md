# Babencoin

In this project, we'll write a blockchain based on a simple proof of work algorithm - Babencoin (all rights reserved, any use of the token without the consent of the owner is prohibited).

## 1. Description of the blockchain

### 1.1. Block Format

Blockchain is a sequence of blocks. An example block from our blockchain (the complete block can be seen in `data/test_block.json`):

```json
{
    "index": 1
    "nonce": 27532,
    reward: 1000
    "issuer": "...",
    "timestamp": 1626003028,
    "max_hash": "...",
    "prev_hash": "...",
    "transactions": [
        {
            amount: 500
            fee: 30
            "comment": "hi",
            "sender": "...",
            "receiver": "...",
            signature: "..."
        }
    ]
}
```

Fields specification:

- `index` - distance from the given block to the genesis block (zero block that is hardcoded into the blockchain).
- `nonce` - an arbitrary number that has no meaning.
- `reward` - the number of babencoins that the one who mined this block receives.
- `issuer` - public RSA key of the one who mined this block (he also receives a `reward`).
- `timestamp` - timestamp of when this block was created.
- `max_hash` - the maximum allowed hash value that this block must have (see 1.3).
- `prev_hash` - hash of the previous block.
- `transactions` - list of transactions of this block. Transaction fields:
  - `amount` - how many babencoins are sent;
  - `fee` - how many babencoins the block miner gets;
  - `comment` - arbitrary string comment;
  - `sender` - public RSA key of the sender of funds;
  - `receiver` - public RSA key of the recipient of funds;
  - `signature` - the signature of the transaction with the sender's private key.

When serialized to JSON, signatures, keys, and hashes are Base64 encoded.

We won't give an exact specification of how the block hash and signature are calculated, and in what format the RSA key is serialized. Consider it implementation-defined in `src/data.rs`.

### 1.2. Protocol

All blockchain nodes form a P2P network - i.e. communication between nodes is symmetric (unlike, for instance, in client-server protocols).

The nodes establish connections using the TCP protocol. Nodes send messages in JSON format. Every two consecutive messages are separated by a zero byte. The maximum size of one message is 64Kb.

There are three types of messages:

1. Block - the sender informs the recipient that there is some valid from the perspective of the sender block. Format:

    ```json
    {
        "kind": "block",
        ... // all block attributes as they are presented in 1.1.
    }
    ```

2. Transaction - the sender informs the recipient that there is some valid transaction from the perspective of the sender that is not yet present in any block. Format:

    ```json
    {
        "kind": "transaction",
        ... // all transaction attributes as they are in 1.1.
    }
    ```

3. Block request - the sender informs the recipient that he wants to obtain a block whose hash is equal to the specified hash. Format:

    ```json
    {
        "kind": "request",
        "block_hash": "..."
    }
    ```

    A fair node, upon receiving such a message, should check whether it has information about such a block, and if so, send this block in response with a message of the first type.

### 1.3. Mining

Any member of the network can add a new block to the blockchain under the following conditions:

1. This block must have a genesis block as an ancestor (it is determined using the `prev_hash` references).
2. Its `timestamp` must be greater than the `timestamp` of the parent block.
3. `reward` must not exceed 1000.
4. All block transactions must be valid:

    - The sender of each transaction must have enough babencoins in the account to pay `amount + fee`.
    - The transaction must have a valid sender's signature.

5. The numerical value of the block hash must not exceed the value of `max_hash`.

    The `max_hash` value is calculated every 16 blocks as follows:

    ```plain
    new_max_hash = old_max_hash * (avg_block_mining_time / target_block_mining_time)
    ```

    Here:

    - `old_max_hash` - `max_hash` value for the previous 16 blocks.
    - `avg_block_mining_time` - average mining time per block over the last 16 blocks.
    - `target_block_mining_time` - 10 seconds.

The miner's task is to choose such a `nonce` so that the block hash does not exceed `max_hash` - then the block will be valid, other participants will accept it and the miner will receive his reward.

A fair miner should mine a new block with `prev_hash` equal to the hash block with the highest `index` among all valid blocks known to this miner. If there're several blocks with the same `index`, the miner should prefer the block which first became known to this miner.

## 2. Node architecture

The node consists of three services, each running in a separate thread, communicating with other services through channels.

1. Peer service - manages TCP connections with other nodes.
2. Gossip service - distributes information about blocks and transactions between nodes.
3. Mining service - mines a new block.

    Scheme of relationships between services:

    ```plain
            TCP streams
                ▲
                │
                ▼
        ┌────────────────┐
        │                │
        │  Peer service  │
        │                │
        └─────┬──────────┘
              │   ▲
    PeerEvent │   │ PeerCommand
              ▼   │
        ┌─────────┴──────┐
        │                │
        │ Gossip service │
        │                │
        └──────┬─────────┘
               │   ▲
    MiningInfo │   │ Block
               ▼   │
        ┌──────────┴─────┐
        │                │
        │ Mining service │
        │                │
        └────────────────┘
    ```

### 2.1. Peer service

The Peer service generates `PeerEvents` and responds to `PeerCommands`.

Events always occur within a session, where a session is a TCP connection. Each new connection is assigned a unique integer session identifier.

Events are of three types:

1. A new session was created (we successfully established or accepted a connection).
2. A new message arrived.
3. Session terminated.

The commands that the peer service responds to are of two types:

1. Send a message within a specific session.
2. Disconnect from the session.

The peer service config consists of the following parameters:

- `dial_addresses` - a list of addresses with which the service will actively try to establish a connection.
- `dial_cooldown` - how long to wait after a failed or disconnected connection attempt before trying to connect to the address again.
- `listen_address` - on which address to listen for incoming connections.

### 2.2. Gossip service

The Gossip service responds to `PeerEvents` sent by the peer service and sends back `PeerCommand`. The gossip service also sets which block the mining service should mine from, and receives mined blocks from it.

The responsibilities of this service are:

1. Handle new sessions from the peer service. Each new session should send the current head block, as well as all pending transactions (transactions that are known but are not added to the blockchain).
2. Process new blocks received from other nodes. Gossip service validates the block, and if it is correct, forwards it to all active sessions with other nodes, who may not know about this block. Also, if the ancestor of the new block is unknown, one should request it from the node from which the new block came.
3. Handle requests for new blocks. If in some session a block request arrives, which is known to this node, the gossip service must send the requested block in this session.
4. Process new transactions. When a new transaction is received, if it is valid, the gossip service must forward it to all active sessions with other nodes that may not know about this transaction.
5. Request unknown blocks. Once in a while, as specified by the `eager_requests_interval` parameter in the config, the gossip service should go through all blocks whose parent is unknown and try to request a parent block from one of the connected nodes. If `eager_requests_interval` is 0, then this functionality is disabled.
6. Set from which block and with which transactions the mining service should mine.
7. Process new blocks received from the mining service. Share the new block to all connected nodes.

### 2.3. Mining service

The mining service receives information from the gossip service about which block to mine and sends successfully mined blocks in response.

The mining service config consists of the following parameters:

- `thread_count` - how many threads to use for mining;
- `max_tx_per_block` - the maximum number of transactions to try to add to a block;
- `public_key` - public RSA key, which should be the issuer of the block.

## 3. Implementation

All the logic of working with the blockchain as a data structure has already been implemented. Namely:

- `src/data.rs` contains block, transaction, and `PeerMessage` definitions. These structures already configured serialization and deserialization using serde and implemented functions for block and transaction validation.
- `src/block_forest.rs` contains the `BlockForest` structure that stores blocks and transactions. The main function of `BlockForest` is the validation of blocks in the entire blockchain and the ability to determine the current "head" block - the block from which mining should be started. `BlockForest` Methods:
  - `head()` - return the current "head" block.
  - `unknown_block_hashes()` - return hashes of all blocks about which `BlockForest` doesn't know anything except they are ancestors of some known blocks. These hashes it is necessary to request in `GossipService` with an interval `eager_requests_interval`.
  - `pending_transactions()` - transactions that are waiting to be added to the blockchain. These transactions should be used when mining.
  - `find_block()` - find the block by hash.
  - `next_max_hash()` - with what `max_hash` should the next block be mined.
  - `add_block()` - tries to add a block to the blockchain. If the validation of this block will fail, the call will return an error.
  - `add_transaction()` - add a transaction to pending transactions. If the sender doesn't have enough funds, returns an error.

You are required to implement only the logic of `PeerService`, `GossipService`, and `MiningService`.

## 4. Hints

- In `PeerService`, you will most likely need two threads per TCP connection: one thread
serves reads, the other - writes.
- To serialize/deserialize messages, use `serde_json::from_str()` and `serde_json::to_writer()`.
- In `GossipService`, use the `select!()` macro from crossbeam to read from multiple channels at the same time.

## 5. Testing

Your service logs are written inside the `test_artifacts` folder, where each test corresponds to its own subdirectory.

If a test fails, the full log of that test will also be printed to stderr after the line:

`=== BEGIN LOGS OF TEST 'test_name' ===`

This may be useful for debugging crashes that don't reproduce well locally.
