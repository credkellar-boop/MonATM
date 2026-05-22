# MonATM Engine 🏧 🤖 🌐

[![Rust](https://img.shields.io/badge/Language-Rust-black?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/badge/CI-GitHub_Actions-blue?style=flat-square&logo=github-actions)](https://github.com/features/actions)
[![Docker Ready](https://img.shields.io/badge/Docker-Ready-2496ED?style=flat-square&logo=docker&logoColor=white)](https://www.docker.com/)

MonATM is a modular, high-performance engine written in Rust for managing secure ATM operations, including fiat-to-crypto off-ramping, secure cash handling, and automated lending.

## Features

* **Hardware Integration:** Interface with cash dispensers, vaults, and secure PIN keypads.
* **Asynchronous Processing:** Built on `tokio` for handling concurrent I/O operations and transactions.
* **Crypto-Ready:** Support for secure wallet generation and blockchain transaction settlement.
* **State Machine:** Robust state management ensures reliable transaction flows.
* **Security First:** Modular authentication and hardware-validated deposit mechanisms.

## Setup

1. Clone the repository.
2. Configure `Config.toml` for your specific hardware paths and RPC endpoints.
3. Build the project:
   ```bash
   cargo build --release

===========================================

monatm/
├── .github/          # CI/CD workflows for testing and validation
├── src/              # Core application source code
│   ├── auth/         # Security & Identity validation
│   ├── crypto/       # Blockchain interface & wallet management
│   ├── hardware/     # Hardware Abstraction Layer (HAL)
│   ├── models/       # Data structures (Account, Ledger, etc.)
│   ├── states/       # Finite State Machine (Idle, PinEntry, etc.)
│   ├── transactions/ # Business logic (Lending, Withdrawals, etc.)
│   ├── controller.rs # Main engine loop & state orchestrator
│   └── main.rs       # Application entry point
├── tests/            # Integration & race-condition tests
├── Cargo.toml        # Dependency & project metadata
├── Config.toml       # Runtime environment & hardware settings
├── Dockerfile        # Multi-stage Docker build configuration
└── docker-compose.yml# Container orchestration configuration
ation
