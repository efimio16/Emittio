# <img src="assets/logo.png" alt="Emittio logo" style="height: 1em"/> Emittio

*Email, without identity. Simple to use, hard to trace.*

> Private messaging without accounts, designed to be as seamless as traditional email.

🌐 [Website](https://emittio.vercel.app/) | 📣 [Telegram](https://t.me/EmittioMail)

## 📫 Why Emittio?

We're building a mail system that is:

- 🔒 **Private by design** — No accounts. End-to-end & post-quantum encrypted. The network cannot see senders or recipients.
- 🕸️ **Decentralized** — owned by users, not by a single company.
- ⚡️ **Seamless** — fast delivery and private synchronization between devices.

## 🧩 Core Components

| Component | Description |
|-|-|
| **Message** | Encrypted content with hidden sender and recipient metadata. |
| **Pointer** | A scan-based locator that lets recipients discover messages without revealing recipient identities. |
| **Event** | Private state records used for mailbox state, network optimization and and synchronization across devices. |
| **Seed** | A single recovery key that derives all addresses, encryption keys, and identities across devices. Replaces traditional accounts. |
| **Address** | A public identifier derived from the Seed that can receive messages without exposing the Seed itself. |
| **Inbox** | A local mailbox that scans the network for messages addressed to a specific Address. |
| **Client** | A user application that connects to the network. |
| **Node** | A network participant that shares storage and availability with others while benefiting from improved performance and synchronization. |

## 🏠 Architecture Overview

### Sender

1. An Address is derived from the sender's Seed.
2. A unique message key is derived from the Address and used to encrypt the message.
3. The encrypted message is published to the network with hidden sender and recipient metadata.
4. A Pointer is created so only the recipient's Inbox can detect the message while scanning.
5. An encrypted Event is published for synchronization across devices without exposing ownership or linkable activity.

### Recipient

1. The recipient's Inbox scans new Pointers for Addresses derived from the Seed.
2. If a matching Pointer is detected, the Inbox records a new Event to avoid scanning it again and synchronize mailbox state.
3. The encrypted message is loaded from the network and decrypted locally.

## 🟢 Current Status

The project is currently in early development of MVP.

### Core

- [x] Cryptography module
- [ ] Network module
- [ ] Node table
- [ ] Multi-hop routing

### Messaging

- [ ] Message publishing and retrieval
- [ ] Pointer discovery
- [ ] Event synchronization

### Mailbox

- [ ] Inbox creation
- [ ] Device synchronization

### Applications

- [ ] Node application
- [ ] Web client


## Quick Start

### Requirements

- Rust (stable) — https://rustup.rs

```bash
cargo run --release
```

## 🤝 Join the Project

Emittio is currently in the R&D stage, and we are starting to form a small team.
If you are interested in:

- privacy-preserving systems
- decentralized networks
- cryptography
- and, in general, creating something new

you can contact us at [emittio@proton.me](mailto:emittio@proton.me).

## 👥 Team

Thanks to everyone who contributed to this project!

- [@mikeyoung3k](https://github.com/mikeyoung3k)

## 🛠️ Tech Stack

### Cryptography
- X25519 and Kyber512 for key encapsulation
- Ed25519 and Dilithium for signatures
- AES-GCM for encryption and decryption
- BLAKE3 for hashes

### Languages
- Rust (nodes and core client)
- Vanilla HTML/CSS/JavaScript (web client)

## ❤️ Support

- BTC: `bc1qq90dh06ah92sg6unfnsn0edx9l9a9msfpagh3f`
- ETH: `0xB9be3CbB7Dc9f9C104640899AeF4A1b4147f9e21`
- SOL: `6evKWq8jEVJS1GiaNp7XhKQqZQg4jzacV96KbGLoHLwV`
- XMR: `87T3MAroNFfBKE7hvUphVFjfYSgaB6a7qVpvrs9hDAcMMJ413bEeyLAe77j7NnfYeF22PPVbwues9C4Ce2z4N7zv3rXE1Do`
