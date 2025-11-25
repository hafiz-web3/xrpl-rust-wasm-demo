# XRPL Rust + WASM Demo

A tiny Rust + WebAssembly demo that simulates XRPL-style ledger logic compiled to WASM.

This repo demonstrates:

- Rust targeting `wasm32-unknown-unknown`
- Exporting functions to JS using `wasm-bindgen`
- A simulated ledger entry with credit / debit operations
- JSON-based serialization similar to XRPL ledger objects
- A structure that can integrate with C++ or JS runtimes

**Ripple job relevance**

> Shows familiarity with Rust, WASM, serialization, and XRPL-style state transitions (for the Senior Software Engineer C++/Rust role).

---

## 🧱 Project structure

```text
xrpl-rust-wasm-demo/
├─ Cargo.toml
└─ src/
   └─ lib.rs
