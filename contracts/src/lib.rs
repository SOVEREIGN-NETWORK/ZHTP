//! Simple token contract
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// Contract state
thread_local! {
    static CONTRACT_STATE: std::cell::RefCell<Option<HashMap<String, u64>>> = std::cell::RefCell::new(None);
}

#[wasm_bindgen]
pub struct TokenContract {
    initialized: bool,
}

#[wasm_bindgen]
impl TokenContract {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self) -> i32 {
        if self.initialized {
            return 0;
        }

        CONTRACT_STATE.with(|state| {
            *state.borrow_mut() = Some(HashMap::from([
                ("owner".to_string(), 1_000_000)
            ]));
        });
        
        self.initialized = true;
        1
    }

    #[wasm_bindgen]
    pub fn transfer(&self, from: &str, to: &str, amount: u64) -> i32 {
        CONTRACT_STATE.with(|state| {
            let mut balances = state.borrow_mut();
            let balances = balances.as_mut().unwrap();
            
            if let Some(&from_balance) = balances.get(from) {
                if from_balance >= amount {
                    balances.insert(from.to_string(), from_balance - amount);
                    *balances.entry(to.to_string()).or_insert(0) += amount;
                    return 1;
                }
            }
            0
        })
    }

    #[wasm_bindgen]
    pub fn balance_of(&self, addr: &str) -> u64 {
        CONTRACT_STATE.with(|state| {
            let balances = state.borrow();
            let balances = balances.as_ref().unwrap();
            *balances.get(addr).unwrap_or(&0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_contract() {
        let mut contract = TokenContract::new();
        
        // Initialize contract
        assert_eq!(contract.init(), 1);
        assert_eq!(contract.init(), 0); // Second init should fail
        
        // Test transfer
        assert_eq!(contract.balance_of("owner"), 1_000_000);
        assert_eq!(contract.transfer("owner", "recipient", 100), 1);
        
        // Verify balances
        assert_eq!(contract.balance_of("recipient"), 100);
        assert_eq!(contract.balance_of("owner"), 999900);
        
        // Test invalid transfer
        assert_eq!(contract.transfer("nonexistent", "recipient", 100), 0);
        assert_eq!(contract.transfer("owner", "recipient", 1_000_000), 0);
    }
}