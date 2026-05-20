# MonAtM

An advanced, high-throughput Rust-based automated financial node system integrating multi-signature state management, hardware interfaces, and real-time oracle consensus.

## Repository Structure

* `.github/workflows/` - CI/CD automation and parallel transaction testing pipelines.
* `src/auth/` - Secure authentication modules and card reader drivers.
* `src/crypto/` - Oracle connections, core wallet logic, and RPC clients.
* `src/hardware/` - Dispenser, deposit vault, and input peripheral interfaces.
* `src/states/` - Asynchronous finite state machine handling pin entry, idle, and idle-wait loops.
* `src/transactions/` - Cash offramps, balances, deposits, and withdrawal logic.

## Prerequisites

* Rust stable toolchain (Edition 2021)
* `libssl-dev` / `pkg-config` (for secure network handshakes)

## Getting Started

1. Clone the repository and configure your environment:
   ```bash
   cp .env.example .env