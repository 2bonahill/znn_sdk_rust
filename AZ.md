# Zenon Rust SDK - Accelerator-Z project proposal


The goal is to closely stick with the SDKs provided by the Zenon Core Dev team. Namely the one for DART and the one for GO:

- https://github.com/zenon-network/znn_sdk_dart
- https://github.com/zenon-network/go-zenon


## Phase 1 - The crypto

This phase will cover all crypto related aspects of the SDK:

### 1.1 The crypto primitives

This will mainly be the adoption of the algorithms involved:
- Argon2 (Key derivation and password hashing)
- EdDSA
- AES
- The Hash functions
- Bech32 for the addresses
- Bip32 (Key structures) and Bip39 (Mnemonic Seed Phrase)
- ...

### 1.2 The Zenon wallet functionality

Goal of this phase is to leverage the crypto primitives and to implement the Zenon wallet functionality:
- KeyPair
- keyStore (setting up the keys, e.g. from mnemonic seed phrases or from entropy)
- KeyFile (storing, encrypting and decrypting the wallets)


## Phase 2 - APIs (JSON-RPC)

The goal of this phase is to enable communication with the Zenon network.

### 2.1 The JSON-RPC client

Setting up the client to communicate with the network. First one will be using websockets as the means of transportation.

### 2.2 The embedded APIs

Next phase will be to implement all the APIs for the smart contracts. This includes setting up the data structures (strong typing in Rust), JSON parsing, etc.

The list will cover:

- embedded.pillar
- embedded.plasma
- embedded.sentinel
- embedded.token
- embedded.swap
- embedded.stake

See https://wiki.zenon.network/#!api.md for the whole list.

### 2.2 The ledger and status APIs

- ledger.getFrontierAccountBlock
- ledger.getUnconfirmedBlocksByAddress
- ledger.getUnreceivedBlocksByAddress
- ledger.getAccountBlockByHash
- ledger.getAccountBlocksByHeight
- ledger.getAccountBlocksByPage
- ledger.getFrontierMomentum
- ledger.getMomentumBeforeTime
- ledger.getMomentumByHash
- ledger.getMomentumsByHeight
- ledger.getMomentumsByPage
- ledger.getDetailedMomentumsByHeight
- ledger.getAccountInfoByAddress

See https://wiki.zenon.network/#!api.md for the whole list.

## Phase 3 - Integration tests

This phase is not a separated one, because for all the features unit tests will be written in parallel. The goal will be to have a near 100% test coverage for the unit tests. On top of that, integration tests will ensure that end-to-end processes work as specified.
