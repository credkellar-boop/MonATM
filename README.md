# MonATM 🪙🏪

[![Rust CI/CD](https://img.shields.io/github/actions/workflow/status/username/MonATM/rust-ci.yml?branch=main&style=for-the-badge&logo=rust)](https://github.com/username/MonATM/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/github/v/release/username/MonATM?style=for-the-badge&color=orange)](https://github.com/username/MonATM/releases)
[![Rust Version](https://img.shields.io/badge/Rust-1.75%2B-black.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

**MonATM** is a high-throughput, async automated financial terminal repo written in Rust. Features a modular State Pattern engine (`Idle`, `PinEntry`, `TxSelect`) to isolate execution states, asynchronous ledger sync, atomic physical inventory tracking for bill dispensers, and polymorphic transaction workers with strict concurrency guards.

---

## 🛠️ System Capabilities & Multi-Crypto Off-Ramp

MonATM natively transforms cryptocurrency volatility into immediate physical liquidity through an isolated multi-tiered engine block:
* **Real-Time Price Locking:** Integrates thread-safe asset oracles to dynamically calculate token fractions with strict slippage controls (max 0.5%).
* **On-Chain Mempool Monitoring:** Runs asynchronous background worker loops tracking multi-chain settlement verification (BTC, ETH, SOL, USDC).
* **Atomic Hardware Handshakes:** Safeguards physical bill dispensing vaults behind a multi-signature software confirmation loop, completely eliminating double-spend hardware exploits.

---

## 📂 Repository File Structure

```text
MonATM/
├── .github/                  # GitHub Actions CI/CD workflows
│   └── workflows/
│       └── rust-ci.yml       # Automated builds, clippy lints, and test pipelines
├── docs/                     # Protocol specifications and architecture states
│   └── state_machine.md      # Documentation of the MonATM lifecycle
├── src/                      # MonATM Core Source Code
│   ├── auth/                 # Validation, card read logic, and verification
│   │   ├── card_reader.rs    # Hardware-level card scanning abstraction
│   │   └── secure_auth.rs    # Verification service handling credential parsing
│   ├── crypto/               # Crypto Liquidity & Network Layer
│   │   ├── client.rs         # Core connection to external liquidity/Exchanges (CEX/DEX)
│   │   ├── oracle.rs         # Real-time exchange rate/ticker feed engine
│   │   ├── wallet.rs         # Hot wallet generation for transaction escrow routing
│   │   └── mod.rs            # Module exposure definitions
│   ├── hardware/             # Physical device abstractions and tracking
│   │   ├── cash_dispenser.rs # Denomination management, inventory, physical dispatch
│   │   ├── deposit_vault.rs  # Validation logic for cash/check intake
│   │   └── input_periph.rs   # Keypad and touchscreen event interfaces
│   ├── models/               # Domain primitives and state data models
│   │   ├── account.rs        # Account balances and routing configurations
│   │   ├── card.rs           # Tokenized card data definitions
│   │   └── ledger.rs         # Local transactional database schema
│   ├── states/               # State Pattern implementation primitives
│   │   ├── crypto_qr.rs      # Generates and displays the invoice QR codes
│   │   ├── crypto_wait.rs    # Asynchronously listens to the mempool for confirmation
│   │   ├── idle.rs           # Waiting for card insertion/crypto interaction event
│   │   ├── pin_entry.rs      # Pin prompt and counter logic
│   │   ├── tx_select.rs      # Main action menu selection loop
│   │   └── mod.rs            # State trait declaration (`NoCard`, `Authenticated`, etc.)
│   ├── transactions/         # Polymorphic execution mechanics
│   │   ├── balance.rs        # Read-only balance ledger check
│   │   ├── crypto_offramp.rs # Custom execution logic swapping token values to fiat
│   │   ├── deposit.rs        # Vault verification and account credit sequence
│   │   ├── withdrawal.rs     # Cash calculation, balance check, and dispenser trigger
│   │   └── mod.rs            # Transaction execution trait definition
│   ├── controller.rs         # Central state machine engine orchestrator
│   └── main.rs               # MonATM binary bootstrap entrypoint
├── tests/                    # Concurrent integration and failure-mode suites
│   ├── state_transition_tests.rs
│   └── transactional_race_tests.rs
├── .gitignore                # Rust template standard ignoring target/ target builds
├── Cargo.toml                # MonATM dependencies, build features, and workspace manifest
└── LICENSE                   # Open-source license documentation