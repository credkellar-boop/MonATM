# MonATM
MonATM is a high-throughput, async automated financial terminal repo written in Rust. Features a modular State Pattern engine (⁠Idle⁠, ⁠PinEntry⁠, ⁠TxSelect⁠) to isolate execution states, asynchronous ledger sync, atomic physical inventory tracking for bill dispensers, and polymorphic transaction workers with strict concurrency guards.
