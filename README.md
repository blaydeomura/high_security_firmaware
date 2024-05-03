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
cargo run -- generate -n <name> --c <cs id> -w .wallet
```

* Sign a file using the specified persona
```
cargo run sign -n <name of signer> -f <file to sign> -o <signed file output name> -w .wallet
```

* Verify a file based on signer and header file
```
cargo run verify -n bob -f bob_test_sig -w .wallet
cargo run verify -n <name of signer> -f <signed data file> -w .wallet
```

* Remove a persona from wallet
```
cargo run remove -n <name> -w .wallet
```

* Note: You will want to remove your signed output file as well if you remove the corresponding name from wallet

## Quantum Example
```
cargo run -- generate -n bob -c 1 -w .wallet
cargo run -- generate -n mallory -c 2 -w .wallet
cargo run -- generate -n dude -c 3 -w .wallet
cargo run -- generate -n alice -c 4 -w .wallet
cargo run -- generate -n shiv -c 5 -w .wallet
```
```
cargo run sign -n bob -f README.md -o bob_test_sig -w .wallet
cargo run sign -n mallory -f README.md -o mallory_test_sig -w .wallet
cargo run sign -n dude -f README.md -o dude_test_sig -w .wallet
cargo run sign -n alice -f README.md -o alice_test_sig -w .wallet
cargo run sign -n shiv -f README.md -o shiv_test_sig -w .wallet
```
```
cargo run verify -n bob -f bob_test_sig -w .wallet
cargo run verify -n mallory -f mallory_test_sig -w .wallet
cargo run verify -n dude -f dude_test_sig -w .wallet
cargo run verify -n alice -f alice_test_sig -w .wallet
cargo run verify -n shiv -f shiv_test_sig -w .wallet
```
```
cargo run remove -n bob -w .wallet
cargo run remove -n mallory -w .wallet
cargo run remove -n dude -w .wallet
cargo run remove -n alice -w .wallet
cargo run remove -n shiv -w .wallet
```
```

```
## Print public key
```
cargo run print-keys -w .wallet
```

## Peer Verify
```
cargo run peer-verify --pk "<Insert key and inclue enclosing brackets>" --file bob_test_sig 
```

## Testing Core Functionality
```
cargo test --test official_tests -- --show-output  
``` 

## Persistence
- Persona data is stored in wallet directory in json format. 
