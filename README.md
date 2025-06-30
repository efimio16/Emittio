# <img src="assets/logo.png" alt="Emittio logo" style="height: 1em"/> Emittio

*Anonymous, decentralized email. Powered by IPFS, quorums, and end-to-end encryption.*

## ✨ Why Emittio?

> A next-gen email protocol designed for privacy, resilience, and full user control.

We’re building a mail system that is:

- 🕸️ **Decentralized** — no central servers, no single point of failure  
- 🔐 **End-to-end encrypted** — only sender and recipient can read the content  
- 👤 **Anonymous** — no metadata linking IPs, addresses, or messages  
- ⚡️ **Efficient** — fast delivery, minimal cost

## 🧠 Architecture Overview

| Component        | Role                                                       |
|------------------|------------------------------------------------------------|
| **IPFS**         | Stores encrypted message blobs                             |
| **Smart Contracts** | Handles decentralized user registration and key anchors |
| **Nodes**        | Relay encrypted content across the network                 |
| **Quorums**      | Act as replicators and pin important data for reliability  |

> 📦 Messages are encrypted and chunked, then routed via multiple nodes using quorum consensus and replication strategies.

## 🛣️ Roadmap

- [ ] **Step 1**: Anonymous delivery via 2-node relay (Tor-like structure)  
- [ ] **Step 2**: Quorum layer with rotation, replication, and caching  
- [ ] **Step 3**: Smart contract-based identity/key registration  
- [ ] **Step 4**: Fully functional, privacy-focused web client  

## 🌍 Status

> MVP design in progress. Community contributors welcome!

## 🧩 Tech Stack (Planned)

- IPFS
- Rust (solana smart contracts)
- Golang (nodes)
- NextJS (web client)

## ❤️ Support

- BTC: `bc1qq90dh06ah92sg6unfnsn0edx9l9a9msfpagh3f`
- ETH: `0xB9be3CbB7Dc9f9C104640899AeF4A1b4147f9e21`
- SOL: `6evKWq8jEVJS1GiaNp7XhKQqZQg4jzacV96KbGLoHLwV`
- XMR: `87T3MAroNFfBKE7hvUphVFjfYSgaB6a7qVpvrs9hDAcMMJ413bEeyLAe77j7NnfYeF22PPVbwues9C4Ce2z4N7zv3rXE1Do`

> ⚠️ Project in early R&D stage. Looking for early feedback & sponsors. Contributors welcome after MVP.