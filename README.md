# XRPL Rust + WASM Demo

A tiny Rust + WebAssembly demo that simulates XRPL-style ledger logic compiled to WASM.

This project is intentionally minimal, designed to demonstrate:
- Rust targeting `wasm32-unknown-unknown`
- Exporting functions to JavaScript using `wasm-bindgen`
- A tiny simulated ledger with `credit` / `debit` state transitions
- JSON-style serialization (Serde) similar to XRPL ledger objects
- A structure that could integrate with C++, JS, or future XRPL-style runtimes

---

## 🚀 Ripple Job Relevance

This repo demonstrates practical familiarity with:
- Rust  
- WebAssembly (WASM)  
- Serialization (Serde / JSON)  
- Ledger-style state transitions  
- Architecture similar to XRPL’s C++/Rust programmability goals  

**This directly supports the Senior Software Engineer C++/Rust role at RippleX.**

---

## 📁 Project Structure

```text
xrpl-rust-wasm-demo/
├── Cargo.toml
└── src/
    └── lib.rs
```

---

## 🛠 How to Build (Rust → WASM)

### 1. Install the WASM target

```bash
rustup target add wasm32-unknown-unknown
```

### 2. Build the WASM artifact

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Output

This will generate a `.wasm` file at:

```text
target/wasm32-unknown-unknown/release/
```

---

## 🧪 Example (Conceptual) JavaScript Usage

```js
import init, { create_ledger, credit, debit } from "./xrpl_rust_wasm_demo.js";

async function main() {
  await init();

  const ledger = create_ledger();

  credit(ledger, 100);
  debit(ledger, 40);

  console.log(JSON.parse(ledger.get_state())); 
}
main();
```

---

## 📜 License

MIT
