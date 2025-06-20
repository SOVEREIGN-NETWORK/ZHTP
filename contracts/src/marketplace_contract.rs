//! Marketplace Smart Contract - Complete WASM Implementation
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketItem {
    pub id: u64,
    pub seller: String,
    pub title: String,
    pub description: String,
    pub price: u64, // In ZHTP tokens
    pub category: String,
    pub condition: String, // "new", "used", "refurbished"
    pub images: Vec<String>, // IPFS hashes
    pub timestamp: u64,
    pub available: bool,
    pub buyer: Option<String>,
    pub shipping_cost: u64,
    pub location: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketTransaction {
    pub id: u64,
    pub item_id: u64,
    pub seller: String,
    pub buyer: String,
    pub price: u64,
    pub timestamp: u64,
    pub status: String, // "pending", "shipped", "delivered", "disputed", "completed"
    pub tracking_info: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserRating {
    pub user: String,
    pub rating: f64,
    pub review: String,
    pub transaction_id: u64,
    pub timestamp: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketUser {
    pub address: String,
    pub username: String,
    pub reputation_score: f64,
    pub total_sales: u64,
    pub total_purchases: u64,
    pub ratings: Vec<UserRating>,
    pub verified_seller: bool,
}

// Global contract state
thread_local! {
    static MARKET_STATE: std::cell::RefCell<Option<MarketContract>> = std::cell::RefCell::new(None);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketContract {
    pub items: HashMap<u64, MarketItem>,
    pub transactions: HashMap<u64, MarketTransaction>,
    pub users: HashMap<String, MarketUser>,
    pub next_item_id: u64,
    pub next_transaction_id: u64,
    pub admin: String,
    pub platform_fee_rate: f64, // Percentage (0.025 = 2.5%)
}

#[wasm_bindgen]
pub struct Marketplace;

#[wasm_bindgen]
impl Marketplace {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen]
    pub fn init(&self, admin_address: &str) -> bool {
        MARKET_STATE.with(|state| {
            *state.borrow_mut() = Some(MarketContract {
                items: HashMap::new(),
                transactions: HashMap::new(),
                users: HashMap::new(),
                next_item_id: 1,
                next_transaction_id: 1,
                admin: admin_address.to_string(),
                platform_fee_rate: 0.025, // 2.5% platform fee
            });
        });
        true
    }

    #[wasm_bindgen]
    pub fn create_user(&self, address: &str, username: &str) -> bool {
        MARKET_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if contract.users.contains_key(address) {
                return false; // User already exists
            }
            
            let user = MarketUser {
                address: address.to_string(),
                username: username.to_string(),
                reputation_score: 5.0, // Starting reputation
                total_sales: 0,
                total_purchases: 0,
                ratings: Vec::new(),
                verified_seller: false,
            };
            
            contract.users.insert(address.to_string(), user);
            true
        })
    }

    #[wasm_bindgen]
    pub fn list_item(&self, seller: &str, title: &str, description: &str, price: u64, category: &str, condition: &str, shipping_cost: u64, location: &str) -> u64 {
        MARKET_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            let item_id = contract.next_item_id;
            
            let item = MarketItem {
                id: item_id,
                seller: seller.to_string(),
                title: title.to_string(),
                description: description.to_string(),
                price,
                category: category.to_string(),
                condition: condition.to_string(),
                images: Vec::new(), // Can be added separately
                timestamp: js_sys::Date::now() as u64,
                available: true,
                buyer: None,
                shipping_cost,
                location: location.to_string(),
            };
            
            contract.items.insert(item_id, item);
            contract.next_item_id += 1;
            
            item_id
        })
    }

    #[wasm_bindgen]
    pub fn purchase_item(&self, item_id: u64, buyer: &str) -> u64 {
        MARKET_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if let Some(item) = contract.items.get_mut(&item_id) {
                if !item.available || item.buyer.is_some() {
                    return 0; // Item not available
                }
                
                let transaction_id = contract.next_transaction_id;
                
                // Create transaction
                let transaction = MarketTransaction {
                    id: transaction_id,
                    item_id,
                    seller: item.seller.clone(),
                    buyer: buyer.to_string(),
                    price: item.price,
                    timestamp: js_sys::Date::now() as u64,
                    status: "pending".to_string(),
                    tracking_info: None,
                };
                
                contract.transactions.insert(transaction_id, transaction);
                
                // Mark item as sold
                item.available = false;
                item.buyer = Some(buyer.to_string());
                
                // Update user stats
                if let Some(seller_user) = contract.users.get_mut(&item.seller) {
                    seller_user.total_sales += 1;
                }
                if let Some(buyer_user) = contract.users.get_mut(buyer) {
                    buyer_user.total_purchases += 1;
                }
                
                contract.next_transaction_id += 1;
                transaction_id
            } else {
                0 // Item not found
            }
        })
    }

    #[wasm_bindgen]
    pub fn update_transaction_status(&self, transaction_id: u64, status: &str, tracking_info: Option<String>) -> bool {
        MARKET_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if let Some(transaction) = contract.transactions.get_mut(&transaction_id) {
                transaction.status = status.to_string();
                transaction.tracking_info = tracking_info;
                true
            } else {
                false
            }
        })
    }

    #[wasm_bindgen]
    pub fn rate_user(&self, rater: &str, rated_user: &str, transaction_id: u64, rating: f64, review: &str) -> bool {
        MARKET_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            // Verify the transaction exists and rater was involved
            if let Some(transaction) = contract.transactions.get(&transaction_id) {
                if transaction.seller != rater && transaction.buyer != rater {
                    return false; // Rater not involved in transaction
                }
            } else {
                return false; // Transaction not found
            }
            
            let user_rating = UserRating {
                user: rater.to_string(),
                rating: rating.max(1.0).min(5.0), // Clamp between 1-5
                review: review.to_string(),
                transaction_id,
                timestamp: js_sys::Date::now() as u64,
            };
            
            if let Some(user) = contract.users.get_mut(rated_user) {
                user.ratings.push(user_rating);
                
                // Recalculate reputation score
                let total_rating: f64 = user.ratings.iter().map(|r| r.rating).sum();
                user.reputation_score = total_rating / user.ratings.len() as f64;
                
                true
            } else {
                false
            }
        })
    }

    #[wasm_bindgen]
    pub fn search_items(&self, query: &str, category: Option<String>, max_price: Option<u64>) -> String {
        MARKET_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let query_lower = query.to_lowercase();
            let matching_items: Vec<&MarketItem> = contract.items.values()
                .filter(|item| {
                    item.available &&
                    (item.title.to_lowercase().contains(&query_lower) ||
                     item.description.to_lowercase().contains(&query_lower)) &&
                    category.as_ref().map_or(true, |cat| &item.category == cat) &&
                    max_price.map_or(true, |max| item.price <= max)
                })
                .collect();
            
            serde_json::to_string(&matching_items).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_item(&self, item_id: u64) -> String {
        MARKET_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            if let Some(item) = contract.items.get(&item_id) {
                serde_json::to_string(item).unwrap_or_default()
            } else {
                "null".to_string()
            }
        })
    }

    #[wasm_bindgen]
    pub fn get_user_items(&self, user: &str) -> String {
        MARKET_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let user_items: Vec<&MarketItem> = contract.items.values()
                .filter(|item| item.seller == user)
                .collect();
            
            serde_json::to_string(&user_items).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_user_transactions(&self, user: &str) -> String {
        MARKET_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let user_transactions: Vec<&MarketTransaction> = contract.transactions.values()
                .filter(|tx| tx.seller == user || tx.buyer == user)
                .collect();
            
            serde_json::to_string(&user_transactions).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_categories(&self) -> String {
        MARKET_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let mut categories: Vec<String> = contract.items.values()
                .map(|item| item.category.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            
            categories.sort();
            serde_json::to_string(&categories).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_stats(&self) -> String {
        MARKET_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let total_items = contract.items.len();
            let available_items = contract.items.values().filter(|i| i.available).count();
            let total_transactions = contract.transactions.len();
            let total_users = contract.users.len();
            let total_volume: u64 = contract.transactions.values().map(|t| t.price).sum();
            
            serde_json::json!({
                "total_items": total_items,
                "available_items": available_items,
                "total_transactions": total_transactions,
                "total_users": total_users,
                "total_volume": total_volume,
                "contract_version": "1.0.0"
            }).to_string()
        })
    }
}
