# Zenon Rust SDK - Accelerator-Z project proposal


The goal is to closely stick with the SDKs provided by the Zenon Core Dev team. Namely the one for DART and the one for GO:

- https://github.com/zenon-network/znn_sdk_dart
- https://github.com/zenon-network/go-zenon


## Phase 1 - The crypto

This phase will cover all crypto related aspects of the SDK:

### 1.1 The crypto primitives and algorithms

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

## Phase 2 - The JSON RPC client

Setting up the client to communicate with the network. First one will be using websockets as the means of transportation.

## Phase 3 - APIs and Models

The goal of this phase is to enable communication with the Zenon network. Goal will be to implement all the APIs for the smart contracts. This includes setting up the data structures (strong typing in Rust), JSON parsing, etc.

The list will cover:

- Embedded
- Ledger
- Stats
- Wallet: to manage wallets through the Node

The full list can be found here: https://wiki.zenon.network/#!api.md

### 3.1 Embedded Endpoints

The embedded endpoints provide endpoints to interact with the NoM embedded smart contracts.

- embedded.pillar (9 endpoints)
- embedded.plasma (3 endpoints)
- embedded.sentinel (5 endpoints)
- embedded.token (3 endpoints)
- embedded.swap (3 endpoints)
- embedded.stake (3 endpoints)

### 3.2 Ledger Endpoints

Ledger provides 13 + 4 (subscribe) endpoints to interact with the NoM dual-ledger.

### 3.3 Stats Endpoints

Stats provides 5 endpoints to examine stats and other information about the Node.



# Time estimation

## Phase 1 (Crypto, Wallet, Key Handling)
=> Crypto primitives and algorithms: 3 days -> 24h
=> Wallet: 1 day -> 8h

> **Total phase 1: 32h**

## Phase 2 (JSON-RPC)
=> 0.5 day -> 4h

> **Total phase 2: 4h**

## Phase 3 (APIs)

There is a total of 48 endpoints to be implemented. For every endpoint there is a data model to be applied to it. Implementing the first two enppoints ...

- znn::api::embedded::Pillar::get_all(...)
- znn::api::Ledger::get_account_info_by_address(...)

took a day, but will be much faster because of the repetition involved. 

Estimation used: 1h per endpoint.

=> 48*1h = +- 50h

> **Total phase 3: 50h**

## Phase 4 - Testing (unit and integration testing)

This phase is not a separated one, because for all the features unit tests will be written in parallel. The goal will be to have a near 100% test coverage for the unit tests. On top of that, integration tests will ensure that end-to-end processes work as specified.

Unit tests will be an integral part, especially for the strongly typed JSON parsing and error handling. 

From experience, testing will be estimated roughly a +25% of the implementation effort.

> **Total phase 4: +25% of development effort**

# Total Effort

- Total implementation effort: 32 + 4 + 50 = 86h
- Testing:  20h

>**=> Total of roughly 106 hours**

As proposed by the community, a generous hourly rate of $100 can be applied.

>**=> 106h * 100 = $10'600**

At the current price of roughly $6 per ZNN this makes: 

>**=> 10'600 / 6 = ZNN 1'700**
