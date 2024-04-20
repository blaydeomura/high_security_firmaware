# Shared repo for High Security Firmware

# Encrypted Wallet Manager

## Overview
- This Rust program is a command-line tool for managing a wallet of ciphersuite objects. A ciphersuite object consists of a name and a key pair generated using quantum safe algorithms. Users can generate and remove ciphersuites as well as sign and verify files using the key pairs stored.

## Features
- Generate: Generate a new key pair using the specified algorithm.
- Remove: Remove an existing key pair from the wallet.
- Sign: Signs a file with a secret key.
- Verify: Verifies a file using public key and signature provided.


## Usage
The program uses the Clap library for parsing command-line arguments. All subcommands support both long and short versions. The available options are as follows:

* View combination of algorithms in each cipher suite
```
cargo run -- algorithms
```

* Generate a new ciphersuite with the specified algorithms
```
cargo run -- generate --name <name> --cs-id <id> --wallet <path to wallet>
```

* Sign a file using the specified persona
```
cargo run -- sign --name <name of signer> --file <file to sign> --output <signed data file> --wallet <path to wallet>
```

* Verify a file based on signer and header file
```
cargo run -- verify --name <name of signer> --file <signed data file> --wallet <path to wallet>
```

* Remove a persona from wallet
```
cargo run -- remove --name <name> --wallet <path to wallet>
```

## Quantum Example
```
cargo run -- generate -n bob -c 1 -w .wallet
```
```
cargo run sign -n bob -f files/file_test.txt -o file_test_sig -w .wallet
```
```
cargo run verify -n bob -f file_test_sig -w .wallet
```
```
cargo run remove -n bob -w .wallet
```

## Non quantum RSA Example signing + verify
```
cargo run -- generate -n mallory -c 5 -w .wallet
```
```
cargo run sign -n mallory -f files/file_test.txt -o mallory_sig_path -w .wallet
```
```
cargo run verify -n mallory -f mallory_sig_path -w .wallet
```

## Testing Core Functionality
```
cargo test --test official_test -- --show-output  
``` 

## Persistence
- Wallet data is stored in wallet file. By convention, this file is called .wallet
