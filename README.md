# Crusty

Crusty is a Rust project showcasing a simple example of Command Query Responsibility Segregation (CQRS) pattern implementation.

## Introduction

Command Query Responsibility Segregation (CQRS) is a design pattern that separates the responsibility for handling commands (write operations) from the responsibility for handling queries (read operations) in an application. This separation can lead to simpler and more scalable systems.

Crusty demonstrates a basic implementation of CQRS in Rust using the following components:

- **EventStore**: Stores events that occur within the system.
- **QueryStore**: Stores data optimized for querying.
- **DB Sync**: Synchronizes the EventStore with the QueryStore, demonstrating the separation of write and read operations.

## Usage

To run the example provided by Crusty, follow these steps:

1. Clone the Crusty repository:
```bash
git clone https://github.com/your_username/crusty.git
```


2. Navigate to the Crusty directory:
```bash
cd src
```

3. Run the example using Cargo:
``` bash
cargo run
```

### Contributing
Contributions to Crusty are welcome! Feel free to open issues or submit pull requests to improve the project.

