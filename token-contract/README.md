# 🌟 Desafio 30k Token - Stellar Soroban Implementation

<div align="center">

<img src="https://assets-global.website-files.com/5deac75ecad2173c2ccccbc7/5dec8960504967fd31147f62_Stellar_lockup_black_RGB.svg" alt="Stellar Logo" width="400"/>

[![Typing SVG](https://readme-typing-svg.herokuapp.com?font=Fira+Code&pause=1000&color=00A3FF&center=true&vCenter=true&width=435&lines=Soroban+Smart+Contract;NearX+Challenge+4.1;Stellar+Blockchain)](https://git.io/typing-svg)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75.0-orange.svg)](https://www.rust-lang.org/)
[![Soroban](https://img.shields.io/badge/Soroban-21.0.0-blue.svg)](https://soroban.stellar.org/)
[![Stellar](https://img.shields.io/badge/Stellar-Mainnet-green.svg)](https://stellar.org)

[<img src="https://img.shields.io/badge/Explore_Contract-2ea44f?style=for-the-badge" />](https://stellar.expert/explorer/public/contract/CD4DJG6ZT7SZLVR4TFJBREFBYMKHPN4OXYLWLDNEVWY7T6XUXIIEQWPE)
[<img src="https://img.shields.io/badge/Documentation-blue?style=for-the-badge" />](https://soroban.stellar.org/docs)
[<img src="https://img.shields.io/badge/NearX_Challenge-red?style=for-the-badge" />](https://nearx.com.br)

</div>

## 🚀 About The Project

<img src="https://raw.githubusercontent.com/Platane/snk/output/github-contribution-grid-snake.svg" alt="snake" style="max-width: 100%;">

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

## ⚡ Features

<div align="center">

[![Typing SVG](https://readme-typing-svg.herokuapp.com?font=Fira+Code&pause=1000&color=FF7F50&center=true&vCenter=true&width=435&lines=Custom+token+initialization;Controlled+minting+system;Secure+transfers;Approval+system;Allowance+management)](https://git.io/typing-svg)

</div>

## 🚀 How to Run

### Prerequisites

```bash
Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Install Soroban CLI
cargo install --locked soroban-cli
Install WASM target
rustup target add wasm32-unknown-unknown
```

### Building

bash
cd token-contract
cargo build --target wasm32-unknown-unknown --release


### Deployment

```bash
stellar contract deploy \
--wasm target/wasm32-unknown-unknown/release/desafio30k.wasm \
--rpc-url https:mainnet.sorobanrpc.com \
--network-passphrase "Public Global Stellar Network ; September 2015" \
--source <YOUR_SECRET_KEY>
```


## 📁 Project Structure

```plaintext
plaintext
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
```

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

# 🔗 Connect With Me


<div align="center">
  
<a href="https://www.linkedin.com/in/vanessabarros-tech/"><img src="https://img.shields.io/badge/linkedin-%231E77B5.svg?&style=for-the-badge&logo=linkedin&logoColor=white" alt="LinkedIn"/></a>
<a href="https://www.instagram.com/vanessabarrostech/"><img src="https://img.shields.io/badge/instagram-%23E4405F.svg?&style=for-the-badge&logo=instagram&logoColor=white" alt="Instagram"/></a>

</div>

<div align="center">

Made with ❤️ by [Vanessa Barros](https://vanbarros76.github.io/)


<img width="100%" src="https://capsule-render.vercel.app/api?type=waving&color=0:FF1493,25:FF00FF,50:4B0082,75:0000FF,100:00FFFF&height=200&section=footer&text=🚀%20🎆%20Launching%20to%20the%20Stars%20🎇%20✨&fontSize=36&fontColor=FFFFFF&animation=twinkling&fontAlignY=65&desc=🌠%20Celebrating%20Stellar%20Smart%20Contract%20Development%20🌠&descSize=20&descAlignY=88" />


</div>

</div>

