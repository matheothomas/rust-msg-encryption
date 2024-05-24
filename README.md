# rust-msg-encryption

## Roadmap
- [x] Implement local network messaging using sockets and threads
- [x] Add server ip address as a parameter
- [ ] Improve the terminal interface
- [ ] Add RSA encryption for the messages

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
