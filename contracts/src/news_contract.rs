//! News Hub Smart Contract - Complete WASM Implementation
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub author: String,
    pub timestamp: u64,
    pub votes: i64,
    pub category: String,
    pub verified: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewsComment {
    pub id: u64,
    pub article_id: u64,
    pub author: String,
    pub content: String,
    pub timestamp: u64,
    pub votes: i64,
}

// Global contract state
thread_local! {
    static NEWS_STATE: std::cell::RefCell<Option<NewsContract>> = std::cell::RefCell::new(None);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewsContract {
    pub articles: HashMap<u64, NewsArticle>,
    pub comments: HashMap<u64, NewsComment>,
    pub next_article_id: u64,
    pub next_comment_id: u64,
    pub verified_authors: HashMap<String, bool>,
    pub admin: String,
}

#[wasm_bindgen]
pub struct NewsHub;

#[wasm_bindgen]
impl NewsHub {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen]
    pub fn init(&self, admin_address: &str) -> bool {
        NEWS_STATE.with(|state| {
            *state.borrow_mut() = Some(NewsContract {
                articles: HashMap::new(),
                comments: HashMap::new(),
                next_article_id: 1,
                next_comment_id: 1,
                verified_authors: HashMap::from([(admin_address.to_string(), true)]),
                admin: admin_address.to_string(),
            });
        });
        true
    }

    #[wasm_bindgen]
    pub fn post_article(&self, title: &str, content: &str, author: &str, category: &str) -> u64 {
        NEWS_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            let article_id = contract.next_article_id;
            let is_verified = contract.verified_authors.get(author).unwrap_or(&false);
            
            let article = NewsArticle {
                id: article_id,
                title: title.to_string(),
                content: content.to_string(),
                author: author.to_string(),
                timestamp: js_sys::Date::now() as u64,
                votes: 0,
                category: category.to_string(),
                verified: *is_verified,
            };
            
            contract.articles.insert(article_id, article);
            contract.next_article_id += 1;
            
            article_id
        })
    }

    #[wasm_bindgen]
    pub fn vote_article(&self, article_id: u64, is_upvote: bool) -> bool {
        NEWS_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if let Some(article) = contract.articles.get_mut(&article_id) {
                if is_upvote {
                    article.votes += 1;
                } else {
                    article.votes -= 1;
                }
                true
            } else {
                false
            }
        })
    }

    #[wasm_bindgen]
    pub fn post_comment(&self, article_id: u64, content: &str, author: &str) -> u64 {
        NEWS_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            let comment_id = contract.next_comment_id;
            
            let comment = NewsComment {
                id: comment_id,
                article_id,
                author: author.to_string(),
                content: content.to_string(),
                timestamp: js_sys::Date::now() as u64,
                votes: 0,
            };
            
            contract.comments.insert(comment_id, comment);
            contract.next_comment_id += 1;
            
            comment_id
        })
    }

    #[wasm_bindgen]
    pub fn get_articles(&self) -> String {
        NEWS_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let mut articles: Vec<&NewsArticle> = contract.articles.values().collect();
            articles.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            
            serde_json::to_string(&articles).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_article(&self, article_id: u64) -> String {
        NEWS_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            if let Some(article) = contract.articles.get(&article_id) {
                serde_json::to_string(article).unwrap_or_default()
            } else {
                "null".to_string()
            }
        })
    }

    #[wasm_bindgen]
    pub fn get_comments(&self, article_id: u64) -> String {
        NEWS_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let comments: Vec<&NewsComment> = contract.comments.values()
                .filter(|c| c.article_id == article_id)
                .collect();
            
            serde_json::to_string(&comments).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn verify_author(&self, author: &str, admin: &str) -> bool {
        NEWS_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if admin == contract.admin {
                contract.verified_authors.insert(author.to_string(), true);
                true
            } else {
                false
            }
        })
    }

    #[wasm_bindgen]
    pub fn get_stats(&self) -> String {
        NEWS_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let total_articles = contract.articles.len();
            let total_comments = contract.comments.len();
            let verified_authors = contract.verified_authors.len();
            let total_votes: i64 = contract.articles.values().map(|a| a.votes).sum();
            
            serde_json::json!({
                "total_articles": total_articles,
                "total_comments": total_comments,
                "verified_authors": verified_authors,
                "total_votes": total_votes,
                "contract_version": "1.0.0"
            }).to_string()
        })
    }
}
