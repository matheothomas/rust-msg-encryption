# rust-msg-encryption
This is a project made in Rust to try and understand the basics of asymmetric encryption, practice Rust and use sockets.

## Roadmap
- [x] Implement local network messaging using sockets and threads
- [x] Add server ip address as a parameter
- [x] Add RSA encryption for the messages
- [ ] Improve the terminal interface

## Principle
For each user, a **private** and a **public** key are generated using the RSA principle.
- The public key is shared with the other user, and will use it to encrypt every message sent.
- The other user will be able to decrypt the message encrypted with its public key using the associated private key.

## Disclaimer
This code should not be used for **security purposes** and is only made to demonstrate how asymmetric encryption works.

## Requirements
- Cargo

## Installation
Clone the repo with :
```bash
git clone https://github.com/matheothomas/rust-msg-encryption
cd rust-msg-encryption/
```

## Run with Cargo

### Server
```bash
cargo run --bin server 172.0.0.1
```

### Client
```bash
cargo run --bin client IP_ADDRESS_SERVER
```

## Credits
The mathematic implementation of the RSA algorithm was highly inspired from [This repository](https://github.com/andrewkiluk/RSA-Library).
