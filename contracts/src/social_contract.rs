//! Social Network Smart Contract - Complete WASM Implementation
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub id: u64,
    pub author: String,
    pub content: String,
    pub timestamp: u64,
    pub likes: u64,
    pub shares: u64,
    pub comments: Vec<u64>,
    pub tags: Vec<String>,
    pub media_hash: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub address: String,
    pub username: String,
    pub bio: String,
    pub followers: Vec<String>,
    pub following: Vec<String>,
    pub posts: Vec<u64>,
    pub verified: bool,
    pub reputation: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SocialComment {
    pub id: u64,
    pub post_id: u64,
    pub author: String,
    pub content: String,
    pub timestamp: u64,
    pub likes: u64,
}

// Global contract state
thread_local! {
    static SOCIAL_STATE: std::cell::RefCell<Option<SocialContract>> = std::cell::RefCell::new(None);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SocialContract {
    pub posts: HashMap<u64, SocialPost>,
    pub users: HashMap<String, UserProfile>,
    pub comments: HashMap<u64, SocialComment>,
    pub next_post_id: u64,
    pub next_comment_id: u64,
    pub admin: String,
}

#[wasm_bindgen]
pub struct SocialNetwork;

#[wasm_bindgen]
impl SocialNetwork {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen]
    pub fn init(&self, admin_address: &str) -> bool {
        SOCIAL_STATE.with(|state| {
            *state.borrow_mut() = Some(SocialContract {
                posts: HashMap::new(),
                users: HashMap::new(),
                comments: HashMap::new(),
                next_post_id: 1,
                next_comment_id: 1,
                admin: admin_address.to_string(),
            });
        });
        true
    }

    #[wasm_bindgen]
    pub fn create_profile(&self, address: &str, username: &str, bio: &str) -> bool {
        SOCIAL_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if contract.users.contains_key(address) {
                return false; // User already exists
            }
            
            let profile = UserProfile {
                address: address.to_string(),
                username: username.to_string(),
                bio: bio.to_string(),
                followers: Vec::new(),
                following: Vec::new(),
                posts: Vec::new(),
                verified: false,
                reputation: 100, // Starting reputation
            };
            
            contract.users.insert(address.to_string(), profile);
            true
        })
    }

    #[wasm_bindgen]
    pub fn create_post(&self, author: &str, content: &str, tags: &str, media_hash: Option<String>) -> u64 {
        SOCIAL_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            let post_id = contract.next_post_id;
            let tag_vec: Vec<String> = if tags.is_empty() {
                Vec::new()
            } else {
                tags.split(',').map(|s| s.trim().to_string()).collect()
            };
            
            let post = SocialPost {
                id: post_id,
                author: author.to_string(),
                content: content.to_string(),
                timestamp: js_sys::Date::now() as u64,
                likes: 0,
                shares: 0,
                comments: Vec::new(),
                tags: tag_vec,
                media_hash,
            };
            
            contract.posts.insert(post_id, post);
            
            // Add post to user's profile
            if let Some(user) = contract.users.get_mut(author) {
                user.posts.push(post_id);
            }
            
            contract.next_post_id += 1;
            post_id
        })
    }

    #[wasm_bindgen]
    pub fn like_post(&self, post_id: u64, user: &str) -> bool {
        SOCIAL_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            if let Some(post) = contract.posts.get_mut(&post_id) {
                post.likes += 1;
                
                // Increase author reputation
                if let Some(author) = contract.users.get_mut(&post.author) {
                    author.reputation += 1;
                }
                true
            } else {
                false
            }
        })
    }

    #[wasm_bindgen]
    pub fn follow_user(&self, follower: &str, following: &str) -> bool {
        SOCIAL_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            // Add to follower's following list
            if let Some(follower_profile) = contract.users.get_mut(follower) {
                if !follower_profile.following.contains(&following.to_string()) {
                    follower_profile.following.push(following.to_string());
                }
            }
            
            // Add to following's followers list
            if let Some(following_profile) = contract.users.get_mut(following) {
                if !following_profile.followers.contains(&follower.to_string()) {
                    following_profile.followers.push(follower.to_string());
                    following_profile.reputation += 5; // Reputation boost for gaining followers
                }
            }
            
            true
        })
    }

    #[wasm_bindgen]
    pub fn post_comment(&self, post_id: u64, author: &str, content: &str) -> u64 {
        SOCIAL_STATE.with(|state| {
            let mut contract = state.borrow_mut();
            let contract = contract.as_mut().unwrap();
            
            let comment_id = contract.next_comment_id;
            
            let comment = SocialComment {
                id: comment_id,
                post_id,
                author: author.to_string(),
                content: content.to_string(),
                timestamp: js_sys::Date::now() as u64,
                likes: 0,
            };
            
            contract.comments.insert(comment_id, comment);
            
            // Add comment to post
            if let Some(post) = contract.posts.get_mut(&post_id) {
                post.comments.push(comment_id);
            }
            
            contract.next_comment_id += 1;
            comment_id
        })
    }

    #[wasm_bindgen]
    pub fn get_feed(&self, user: &str, limit: usize) -> String {
        SOCIAL_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let mut posts: Vec<&SocialPost> = contract.posts.values().collect();
            posts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            
            let limited_posts: Vec<&SocialPost> = posts.into_iter().take(limit).collect();
            serde_json::to_string(&limited_posts).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_user_profile(&self, address: &str) -> String {
        SOCIAL_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            if let Some(profile) = contract.users.get(address) {
                serde_json::to_string(profile).unwrap_or_default()
            } else {
                "null".to_string()
            }
        })
    }

    #[wasm_bindgen]
    pub fn get_post(&self, post_id: u64) -> String {
        SOCIAL_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            if let Some(post) = contract.posts.get(&post_id) {
                serde_json::to_string(post).unwrap_or_default()
            } else {
                "null".to_string()
            }
        })
    }

    #[wasm_bindgen]
    pub fn search_posts(&self, query: &str) -> String {
        SOCIAL_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let query_lower = query.to_lowercase();
            let matching_posts: Vec<&SocialPost> = contract.posts.values()
                .filter(|post| {
                    post.content.to_lowercase().contains(&query_lower) ||
                    post.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
                })
                .collect();
            
            serde_json::to_string(&matching_posts).unwrap_or_default()
        })
    }

    #[wasm_bindgen]
    pub fn get_stats(&self) -> String {
        SOCIAL_STATE.with(|state| {
            let contract = state.borrow();
            let contract = contract.as_ref().unwrap();
            
            let total_posts = contract.posts.len();
            let total_users = contract.users.len();
            let total_comments = contract.comments.len();
            let total_likes: u64 = contract.posts.values().map(|p| p.likes).sum();
            
            serde_json::json!({
                "total_posts": total_posts,
                "total_users": total_users,
                "total_comments": total_comments,
                "total_likes": total_likes,
                "contract_version": "1.0.0"
            }).to_string()
        })
    }
}
