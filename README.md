# DistributedLedger: Decentralized Data Provenance with Rust

This repository houses a suite of prototypes exploring innovative consensus mechanisms and zero-knowledge proof (ZKP) integrations for verifiable, trustless data provenance. It aims to provide a foundation for building robust and transparent distributed ledgers using the Rust programming language, focusing on security, performance, and privacy.

The project directly addresses the growing need for secure and auditable data trails across various industries, including supply chain management, healthcare, and digital identity. By leveraging the power of Rust's memory safety and performance, coupled with cutting-edge cryptographic techniques, DistributedLedger offers a platform for creating decentralized applications that guarantee data integrity and authenticity. The emphasis on zero-knowledge proofs allows for data verification without revealing the underlying information, enabling privacy-preserving solutions.

This repository is not intended to be a production-ready distributed ledger in its entirety, but rather a collection of experimental modules and implementations that can be adapted and integrated into existing systems or used as a blueprint for building new ones. The focus is on exploring different approaches to consensus, data storage, and verification, providing developers with a flexible toolkit for prototyping and experimenting with decentralized ledger technologies. We prioritize modularity and clear documentation to facilitate understanding and contribution.

The ultimate goal is to contribute to the broader ecosystem of decentralized technologies by offering a set of well-documented, high-performance, and secure building blocks for creating verifiable and trustless data provenance solutions. We invite developers, researchers, and enthusiasts to explore the code, contribute their expertise, and help us advance the state of the art in distributed ledger technology.

## Key Features

*   **Pluggable Consensus Mechanisms:** Implementations of various consensus algorithms, including Proof-of-Stake (PoS) and Practical Byzantine Fault Tolerance (pBFT), allowing for experimentation with different trade-offs between throughput, latency, and fault tolerance. The implementation includes an abstract `Consensus` trait in Rust, which allows developers to easily integrate other consensus mechanisms.
*   **Zero-Knowledge Proof (ZKP) Integration:** Seamless integration of ZKP libraries like `arkworks` for proving data integrity without revealing the underlying data. This includes example circuits demonstrating how to generate and verify proofs for specific data operations. The codebase leverages the capabilities of ZKPs to ensure data veracity in a privacy-preserving manner.
*   **Merkle Tree Data Structures:** Efficient and secure data storage using Merkle trees, enabling efficient verification of data integrity and membership proofs. The Merkle tree implementation is optimized for performance and memory usage, crucial for large-scale data storage.
*   **Blockchain Data Structure:** A basic blockchain implementation that serves as the foundation for the distributed ledger. This includes functionalities for creating, validating, and appending blocks to the chain, ensuring the immutability of the data. Includes configurable block size and transaction capacity.
*   **Modular Architecture:** The codebase is designed with a modular architecture, allowing developers to easily add or modify components without affecting the overall system. This facilitates experimentation and customization.
*   **REST API Layer:** Basic REST API built using `actix-web` to interact with the ledger, allowing external applications to easily access and manipulate data. This includes endpoints for adding data, querying the ledger, and verifying data integrity.
*   **Data Serialization:** Implemented data serialization and deserialization routines using `serde`, to ensure data integrity and portability. This allows the project to integrate with external systems more easily.

## Technology Stack

*   **Rust:** The primary programming language used for its memory safety, performance, and suitability for building secure and reliable systems.
*   **tokio:** Asynchronous runtime for Rust, used for handling concurrent operations and network communication.
*   **actix-web:** A powerful, pragmatic, and extremely fast web framework for Rust, used for building the REST API.
*   **serde:** A popular serialization framework for Rust, used for serializing and deserializing data structures.
*   **arkworks:** A Rust library for zero-knowledge proofs and other cryptographic primitives.
*   **rocksdb:** Embedded persistent key-value store for storing the ledger data.

## Installation

1.  **Install Rust:** Ensure you have Rust and Cargo installed. The recommended way to install Rust is using `rustup`:
    
2.  **Clone the Repository:** Clone the DistributedLedger repository from GitHub:
    
3.  **Build the Project:** Build the project using Cargo:
    
    For optimized performance, use the release build:
    

## Configuration

The DistributedLedger application can be configured using environment variables. These variables control various aspects of the application's behavior, such as the listening port, database path, and consensus settings.

*   `LEDGER_PORT`: The port on which the REST API will listen for incoming requests. Defaults to `8080`.
*   `LEDGER_DATABASE_PATH`: The path to the RocksDB database where the ledger data will be stored. Defaults to `./ledger_data`.
*   `CONSENSUS_ALGORITHM`: Specifies the consensus algorithm to use. Supported values are `PoS` and `pBFT`. Defaults to `PoS`.
*   `GENESIS_BLOCK_DATA`: Initial data for the genesis block. Defaults to "Genesis Block".
*   `PRIVATE_KEY`: The node's private key used for signing transactions and participating in consensus. If not provided, a random key will be generated.

To set these environment variables, you can use the `export` command in your terminal:

export LEDGER_PORT=8081
export LEDGER_DATABASE_PATH=/path/to/my/ledger_data
export CONSENSUS_ALGORITHM=pBFT

## Usage

1.  **Run the Application:** After building the project, you can run it using Cargo:
    
    For the release build:
    
2.  **Interact with the REST API:** The application provides a REST API for interacting with the ledger. Here are some example requests:

    *   **Add Data:**
        curl -X POST -H "Content-Type: application/json" -d '{"data": "New data"}' http://localhost:8080/add_data

    *   **Get Data by Index:**
        curl http://localhost:8080/get_data/0

    *   **Get Latest Block:**
        curl http://localhost:8080/get_latest_block

    * API documentation will be provided as a separate openAPI specification file later.

## Contributing

We welcome contributions to DistributedLedger! Please follow these guidelines:

1.  Fork the repository and create a branch for your feature or bug fix.
2.  Write clear and concise code with detailed comments.
3.  Include unit tests for your changes.
4.  Submit a pull request with a clear description of your changes.
5.  Follow the Rust coding style guidelines.
6.  Ensure that all tests pass before submitting your pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/DistributedLedger/blob/main/LICENSE) file for details.

## Acknowledgements

This project leverages the excellent work of the Rust community and the developers of the various libraries used within it. We are grateful for their contributions.