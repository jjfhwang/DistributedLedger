# DistributedLedger: A Rust-Based Minimalist Distributed Ledger Implementation

This repository contains a highly optimized and secure implementation of a distributed ledger, built from the ground up using Rust. Designed for simplicity and performance, this ledger aims to provide a foundational building block for various decentralized applications, focusing on verifiable data storage and consensus mechanisms. The project prioritizes clear architectural design, rigorous testing, and comprehensive documentation, making it an ideal platform for experimentation and integration into larger systems.

This distributed ledger provides a robust and tamper-proof system for recording transactions across a network of participating nodes. It leverages cryptographic primitives for data integrity and utilizes a simplified consensus algorithm to ensure agreement on the order and validity of transactions. Unlike many complex blockchain implementations, this project minimizes external dependencies and focuses on core ledger functionalities. This allows for easier auditing, customization, and integration into resource-constrained environments. The goal is to create a ledger that is auditable and easy to understand, empowering developers to build their own secure and decentralized applications without the overhead of complex blockchain frameworks. By using Rust, we ensure memory safety and performance, crucial for distributed systems operating under high load.

The primary benefits of this distributed ledger include its inherent security due to Rust's memory safety features, its high performance stemming from Rust's efficient compilation and execution, and its ease of understanding due to its minimalist design. This makes it well-suited for applications requiring a reliable and auditable record of transactions, such as supply chain management, data provenance tracking, or identity management systems. Furthermore, the modular architecture allows for easy extension with custom transaction types, consensus algorithms, and storage backends. The ledger provides a foundation upon which specialized decentralized applications can be built without compromising on security or performance.

## Key Features

*   **Transaction Management:** Supports the creation, validation, and storage of transactions. Transactions are digitally signed to ensure authenticity and prevent tampering. The transaction structure includes fields for sender, receiver, amount, and timestamp, all hashed cryptographically.
*   **Block Creation and Chaining:** Groups transactions into blocks, which are then linked together chronologically using cryptographic hashes, forming an immutable chain. Each block contains a header with metadata, including the hash of the previous block, a timestamp, and a Merkle root of the transactions within the block.
*   **Simplified Consensus Mechanism:** Implements a Proof-of-Authority (PoA) consensus algorithm for simplicity and efficiency. In PoA, designated authorities validate and add blocks to the chain.
*   **Merkle Tree Implementation:** Utilizes Merkle trees for efficient verification of transaction inclusion within a block. This allows for selectively proving the existence of specific transactions without requiring the entire block to be downloaded.
*   **Cryptographic Security:** Employs SHA-256 hashing for data integrity and Ed25519 signatures for transaction authentication, ensuring strong security against forgery and manipulation.
*   **Modular Design:** Allows for easy swapping of consensus algorithms, storage backends, and transaction types. This modularity provides flexibility to adapt the ledger to specific application requirements.
*   **Persistent Storage:** Implements a simple file-based storage backend for storing the ledger data. This can be easily replaced with a more robust database system for production environments.

## Technology Stack

*   **Rust:** The core programming language, providing memory safety, performance, and concurrency.
*   **SHA-256:** Used for hashing blocks and transactions to ensure data integrity. Implemented using the `sha2` crate.
*   **Ed25519:** Used for creating and verifying digital signatures for transactions, providing strong authentication. Implemented using the `ed25519-dalek` crate.
*   **Merkle Trees:** Used for efficient verification of transaction inclusion within blocks. A custom merkle tree implementation is included.
*   **Tokio:** An asynchronous runtime for Rust, used to handle network communication and concurrent operations.

## Installation

1.  **Install Rust:** Ensure you have Rust installed. You can download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Follow the instructions for your operating system.

2.  **Clone the Repository:** Clone this repository to your local machine:
    `git clone https://github.com/jjfhwang/DistributedLedger.git`

3.  **Navigate to the Project Directory:** Change your current directory to the cloned repository:
    `cd DistributedLedger`

4.  **Build the Project:** Use the `cargo build` command to compile the project:
    `cargo build`

5.  **Run the Tests:** Verify the installation by running the unit tests:
    `cargo test`

## Configuration

The application can be configured through environment variables. The following variables are supported:

*   `NODE_ID`: A unique identifier for the node. Defaults to "node1".
*   `STORAGE_PATH`: The directory where the ledger data will be stored. Defaults to "./ledger_data".
*   `LISTEN_ADDRESS`: The address the node will listen on for incoming connections. Defaults to "127.0.0.1:8080".
*   `AUTHORITY_PUBLIC_KEY`: The public key of the authority node. This is necessary for nodes validating blocks in the Proof-of-Authority consensus.
*   `AUTHORITY_PRIVATE_KEY`: The private key of the authority node. This is necessary for nodes creating blocks in the Proof-of-Authority consensus.

To set environment variables, use the appropriate method for your operating system. For example, in Linux or macOS, you can use the `export` command:

`export NODE_ID=my_node`
`export STORAGE_PATH=/path/to/my/ledger_data`

## Usage

After successful installation and configuration, you can run the distributed ledger node using the `cargo run` command:

`cargo run`

This will start the node, which will listen for incoming connections on the configured address. The node can then participate in the distributed ledger network by creating and validating transactions.

An example on how to create a transaction would look like this (using the command line interface, assuming you have implemented one as part of your project):
`./distributed_ledger_cli create_transaction --sender sender_address --receiver receiver_address --amount 10`

This creates a transaction and signs it with the node's private key. This transaction is then broadcast to the network.

The API (assuming one is implemented as part of the project) for interacting with the ledger would allow you to fetch blocks. For example:
`curl http://127.0.0.1:8080/blocks/latest`

This would retrieve the latest block in the chain.

## Contributing

We welcome contributions to this project! Please follow these guidelines when contributing:

*   Fork the repository and create a new branch for your feature or bug fix.
*   Write clear and concise commit messages.
*   Submit a pull request with a detailed description of your changes.
*   Ensure your code adheres to the Rust coding style.
*   Include unit tests for your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/DistributedLedger/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to acknowledge the Rust community for their invaluable support and resources. We also thank the developers of the various crates used in this project for their excellent libraries.