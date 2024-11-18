# 🌟 Desafio 30k Token - Stellar Soroban Implementation

<div align="center">
  
<img src="https://assets-global.website-files.com/5deac75ecad2173c2ccccbc7/5dec8960504967fd31147f62_Stellar_lockup_black_RGB.svg" alt="Stellar Logo" width="400"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75.0-orange.svg)](https://www.rust-lang.org/)
[![Soroban](https://img.shields.io/badge/Soroban-21.0.0-blue.svg)](https://soroban.stellar.org/)
[![Stellar](https://img.shields.io/badge/Stellar-Mainnet-green.svg)](https://stellar.org)

[Explore Contract](https://stellar.expert/explorer/public/contract/CD4DJG6ZT7SZLVR4TFJBREFBYMKHPN4OXYLWLDNEVWY7T6XUXIIEQWPE) •
[Documentation](https://soroban.stellar.org/docs) •
[NearX Challenge](https://nearx.com.br)

</div>

## 🚀 About The Project

This project is part of NearX Challenge 4.1, focused on token development on the Stellar blockchain using the Soroban framework. The contract implements a complete token with minting, transfer, and approval functionalities.

### 📊 Token Information

<div align="center">

| Feature | Value |
|---------------|-------|
| Name | Desafio 30k |
| Symbol | D30K |
| Decimals | 7 |
| Contract ID | `CD4DJG6ZT7SZLVR4TFJBREFBYMKHPN4OXYLWLDNEVWY7T6XUXIIEQWPE` |
| Network | Mainnet |

</div>

## 🛠️ Technologies Used

- **Rust** - Main programming language
- **Soroban SDK** - Smart contract development framework
- **Stellar Network** - Base blockchain
- **Cargo** - Dependency manager
- **Git** - Version control

## 🔥 Features

- ✨ Custom token initialization
- 💰 Controlled minting system
- 📤 Secure transfers
- 🔐 Approval system
- 💼 Allowance management
- 📊 Balance queries

## 🚀 How to Run

### Prerequisites

<div align="left">

bash
Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Install Soroban CLI
cargo install --locked soroban-cli
Install WASM target
rustup target add wasm32-unknown-unknown

</div>

### Building

bash
cd token-contract
cargo build --target wasm32-unknown-unknown --release


### Deployment

bash
stellar contract deploy \
--wasm target/wasm32-unknown-unknown/release/desafio30k.wasm \
--rpc-url https:mainnet.sorobanrpc.com \
--network-passphrase "Public Global Stellar Network ; September 2015" \
--source <YOUR_SECRET_KEY>


## 📁 Project Structure

<div align="left">

token-contract/
├── 📂 contracts/
│ └── 📂 desafio30k/
│ ├── 📂 src/
│ │ ├── 📄 lib.rs # Core contract implementation
│ │ ├── 📄 admin.rs # Admin functionality
│ │ ├── 📄 storage.rs # State management
│ │ └── 📄 token.rs # Token operations
│ └── 📄 Cargo.toml
└── 📄 Cargo.toml

</div>

## 🔍 Important Links

- [Deployment Transaction](https://stellar.expert/explorer/public/tx/832f94592f2a847edca4e5cbe2a2b18fc057dee5ee3cd39be8bf1814b425dbdd)
- [Contract on Explorer](https://stellar.expert/explorer/public/contract/CD4DJG6ZT7SZLVR4TFJBREFBYMKHPN4OXYLWLDNEVWY7T6XUXIIEQWPE)
- [Soroban Documentation](https://soroban.stellar.org/)
- [Stellar Documentation](https://developers.stellar.org/)

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are always welcome! Please read the [contribution guide](CONTRIBUTING.md) first.

## 🙏 Acknowledgments

- NearX for the opportunity and mentorship
- Stellar community for support
- All developers who contributed with feedback

<div align="center">

Made with ❤️ by [Vanessa Barros](https://www.linkedin.com/in/vanessabarros-tech/)

</div>

