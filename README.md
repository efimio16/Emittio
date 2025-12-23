# <img src="assets/logo.png" alt="Emittio logo" style="height: 1em"/> Emittio

*Anonymous, decentralized email. Powered by IPFS & end-to-end encryption.*

🌐 [Website](https://emittio.vercel.app/) | 📣 [Telegram](https://t.me/EmittioMail)

## Why Emittio?

> A next-gen email protocol designed for anonymity, resilience, and full user control.

We're building a mail system that is:

- 🕸️ **Decentralized** — no central servers, no single point of failure.
- 🔐 **End-to-end encrypted** — only sender and recipient can read the content.
- 👤 **Anonymous** — no IDs or accounts
- ⚡️ **Efficient** — fast delivery

## 🏠 Architecture

With other words, "how".

| Component | Role |
|-|-|
| **Inbox** | Public key representing a mail addresses |
| **Session** | Basicly, it's a seed from which are derived inbox keys. Sessions are never stored in the network, meaning there aren't accounts in classical sense |
| **Tags** | Small parts of data on network representing encrypted states such as last refresh time or a pointer to an envelope. These are the main optimization |
| **Envelope** | This is an anonymous version of the mail, i.e. there's all encrypted including metadata: sender, recipient and time. But for an efficient lookup, each envelope has a pointer: a hash of shared secret of the recipient and the envelope (not the sender). |
| **DHT** | Allows to store and find encrypted mails in network |
| **Mixnet** | Routes onion-like data through 3 hops (prevents IP correlation) and sends a lot of dummy requests (partially prevents timing correlation) |
| **Nodes** | Save encrypted content and forward onion-like messages |
| **Quorums** | Pseudo-random groups of nodes that temporarily caches envelope's pointer to a group of inboxes. In this way client checks only part of envelopes instead of scanning the whole network |

## 🛣️ Roadmap

1. [x] Alice-Bob cryptography system prototype in Rust
2. [x] `Client`, `Node` and `MockTransport` abstractions
3. [x] Tags, their storage and lookup
4. [ ] DHT
5. [ ] QUIC and/or TCP transports
6. [ ] Mixnet (onion-like routing & dummy requests)
7. [ ] Quorums
8. [ ] UX & UI prototyping
9. [ ] Svelte web client

## 🟢 Status

> MVP design in progress. Community contributors welcome!

## 🛠️ Tech Stack

- X25519 and Kyber512 for shared secret
- Ed25519 and dilithium for signatures
- ChaCha20Poly1305 for envelopes and AES-GCM for onion traffic and tags
- BLAKE3 for hashes and SHA256 for PoW hashes
- DHT on QUIC protocol
- Rust (nodes)
- Svelte + WASM (web client)

## ❤️ Support

- BTC: `bc1qq90dh06ah92sg6unfnsn0edx9l9a9msfpagh3f`
- ETH: `0xB9be3CbB7Dc9f9C104640899AeF4A1b4147f9e21`
- SOL: `6evKWq8jEVJS1GiaNp7XhKQqZQg4jzacV96KbGLoHLwV`
- XMR: `87T3MAroNFfBKE7hvUphVFjfYSgaB6a7qVpvrs9hDAcMMJ413bEeyLAe77j7NnfYeF22PPVbwues9C4Ce2z4N7zv3rXE1Do`

> ⚠️ Project in early R&D stage. Looking for early feedback & sponsors. Contributors welcome after MVP.