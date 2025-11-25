use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// A tiny XRPL-style ledger entry
#[derive(Serialize, Deserialize)]
pub struct LedgerEntry {
    pub account: String,
    pub balance: u64,
}

/// Create a new ledger entry (simulating something XRPL would keep on-chain)
#[wasm_bindgen]
pub fn create_entry(account: &str, balance: u64) -> String {
    let entry = LedgerEntry {
        account: account.to_string(),
        balance,
    };
    serde_json::to_string(&entry).unwrap()
}

/// Add (credit) balance — simple demo of state update
#[wasm_bindgen]
pub fn credit(entry_json: &str, amount: u64) -> String {
    let mut entry: LedgerEntry = serde_json::from_str(entry_json).unwrap();
    entry.balance += amount;
    serde_json::to_string(&entry).unwrap()
}

/// Subtract (debit) balance — with underflow protection
#[wasm_bindgen]
pub fn debit(entry_json: &str, amount: u64) -> String {
    let mut entry: LedgerEntry = serde_json::from_str(entry_json).unwrap();

    if amount > entry.balance {
        // mimic XRPL-style invariant check
        return "ERROR: insufficient balance".to_string();
    }

    entry.balance -= amount;
    serde_json::to_string(&entry).unwrap()
}
