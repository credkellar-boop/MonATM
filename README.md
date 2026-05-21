# MonATM Engine🏧🤖🌐

[![Rust](https://img.shields.io/badge/language-Rust-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![CI](https://github.com/monatm/engine/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/monatm/engine/actions)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](docker-compose.yml)

MonATM is a modular, high-performance engine written in Rust for managing secure ATM operations, including fiat-to-crypto off-ramping, secure cash handling, and automated lending.

## Features
- **Hardware Integration:** Interface with cash dispensers, vaults, and secure PIN keypads.
- **Asynchronous Processing:** Built on `tokio` for handling concurrent I/O operations and transactions.
- **Crypto-Ready:** Support for secure wallet generation and blockchain transaction settlement.
- **State Machine:** Robust state management ensures reliable transaction flows.
- **Security First:** Modular authentication and hardware-validated deposit mechanisms.

## Setup
1. Clone the repository.
2. Configure `Config.toml` for your specific hardware paths and RPC endpoints.
3. Build the project:
   ```bash
   cargo build --release

docker-compose up --build

cargo test

monatm/
├── .github/          # CI/CD workflows for automated testing
├── src/              # Main application source code
│   ├── auth/         # Security & Identity (PIN validation)
│   ├── crypto/       # Blockchain interface & Wallet management
│   ├── hardware/     # Hardware Abstraction Layer (HAL)
│   ├── models/       # Data structures (Account, Ledger, Card)
│   ├── states/       # Finite State Machine (Transaction flows)
│   ├── transactions/ # Business logic (Lending, Off-ramps)
│   ├── controller.rs # Main engine loop & state machine orchestrator
│   ├── deposit.rs    # High-level deposit handler
│   └── main.rs       # Application entry point
├── tests/            # Integration & race-condition tests
├── Cargo.toml        # Dependency & project configuration
├── Config.toml       # Runtime environment & hardware settings
├── Dockerfile        # Containerization build steps
└── docker-compose.yml# Deployment orchestration
