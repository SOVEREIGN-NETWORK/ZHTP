use wasm_bindgen::prelude::*;

// Set up panic hook for better error messages in debug mode
#[cfg(all(feature = "console_error_panic_hook", debug_assertions))]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Basic contract execution function - returns JSON string
#[wasm_bindgen]
pub fn execute_contract(contract_type: &str, method: &str, _params: &str) -> String {
    match contract_type {
        "marketplace" => {
            match method {
                "list_item" => r#"{"success": true, "data": "Item listed successfully"}"#.to_string(),
                "buy_item" => r#"{"success": true, "data": "Item purchased successfully"}"#.to_string(),
                "get_items" => r#"{"success": true, "data": "[]"}"#.to_string(),
                _ => r#"{"success": false, "data": "Unknown marketplace method"}"#.to_string(),
            }
        },
        "news" => {
            match method {
                "post_article" => r#"{"success": true, "data": "Article posted successfully"}"#.to_string(),
                "vote" => r#"{"success": true, "data": "Vote recorded"}"#.to_string(),
                "get_articles" => r#"{"success": true, "data": "[]"}"#.to_string(),
                _ => r#"{"success": false, "data": "Unknown news method"}"#.to_string(),
            }
        },
        "social" => {
            match method {
                "post_message" => r#"{"success": true, "data": "Message posted successfully"}"#.to_string(),
                "follow_user" => r#"{"success": true, "data": "User followed"}"#.to_string(),
                "get_feed" => r#"{"success": true, "data": "[]"}"#.to_string(),
                _ => r#"{"success": false, "data": "Unknown social method"}"#.to_string(),
            }
        },
        _ => r#"{"success": false, "data": "Unknown contract type"}"#.to_string(),
    }
}

/// Validate contract bytecode
#[wasm_bindgen]
pub fn validate_contract(bytecode: &[u8]) -> bool {
    // Basic validation - check if it's valid WASM
    bytecode.len() > 8 && &bytecode[0..4] == b"\0asm"
}

/// Simple test function
#[wasm_bindgen]
pub fn test_contract() -> String {
    "ZHTP Contracts WASM module loaded successfully".to_string()
}