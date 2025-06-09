use crate::storage::content::{ContentId, ContentMetadata};
use std::{
    collections::{HashMap, HashSet, BTreeMap},
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::RwLock;
use anyhow::Result;

/// Node discovery service
pub struct DiscoveryNode {
    addr: SocketAddr,
    nodes: Arc<RwLock<HashMap<SocketAddr, String>>>,
    content_index: ContentIndex,
    ready: bool,
}

impl DiscoveryNode {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        Ok(Self {
            addr,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            content_index: ContentIndex::new(),
            ready: false,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        self.ready = true;
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn get_address(&self) -> SocketAddr {
        self.addr
    }

    pub async fn register_node(&mut self, addr: SocketAddr, name: String) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.insert(addr, name);
        Ok(())
    }

    pub async fn find_nodes(&self, name_prefix: String) -> Result<Vec<SocketAddr>> {
        let nodes = self.nodes.read().await;
        let mut matches = Vec::new();
        
        for (addr, node_name) in nodes.iter() {
            if node_name.starts_with(&name_prefix) {
                matches.push(*addr);
            }
        }
        
        Ok(matches)
    }
}

/// Content metadata index for efficient searching
#[derive(Debug)]
pub struct ContentIndex {
    /// Index by content type
    type_index: Arc<RwLock<HashMap<String, HashSet<ContentId>>>>,
    /// Index by size ranges (in KB)
    size_index: Arc<RwLock<BTreeMap<u64, HashSet<ContentId>>>>,
    /// Index by tags/metadata
    tag_index: Arc<RwLock<HashMap<String, HashSet<ContentId>>>>,
}

impl ContentIndex {
    pub fn new() -> Self {
        Self {
            type_index: Arc::new(RwLock::new(HashMap::new())),
            size_index: Arc::new(RwLock::new(BTreeMap::new())),
            tag_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Index content metadata for efficient search
    pub async fn index_content(&self, id: ContentId, metadata: &ContentMetadata) {
        // Index by content type
        {
            let mut types = self.type_index.write().await;
            types.entry(metadata.content_type.clone())
                .or_insert_with(HashSet::new)
                .insert(id.clone());
        }

        // Index by size range (in KB)
        {
            let size_kb = metadata.size / 1024;
            let mut sizes = self.size_index.write().await;
            sizes.entry(size_kb)
                .or_insert_with(HashSet::new)
                .insert(id.clone());
        }

        // Index all tags from metadata
        let mut tag_idx = self.tag_index.write().await;
        for tag in &metadata.tags {
            tag_idx.entry(tag.clone())
                .or_insert_with(HashSet::new)
                .insert(id.clone());
        }
    }

    /// Search content by type
    pub async fn search_by_type(&self, content_type: &str) -> HashSet<ContentId> {
        let types = self.type_index.read().await;
        types.get(content_type)
            .cloned()
            .unwrap_or_default()
    }

    /// Search content by size range (in KB)
    pub async fn search_by_size(&self, min_kb: u64, max_kb: u64) -> HashSet<ContentId> {
        let sizes = self.size_index.read().await;
        let mut results = HashSet::new();
        
        for (_, ids) in sizes.range(min_kb..=max_kb) {
            results.extend(ids.iter().cloned());
        }
        results
    }

    /// Search content by tag
    pub async fn search_by_tag(&self, tag: &str) -> HashSet<ContentId> {
        let tags = self.tag_index.read().await;
        tags.get(tag)
            .cloned()
            .unwrap_or_default()
    }

    /// Remove content from all indices
    pub async fn remove_content(&self, id: &ContentId) {
        let mut types = self.type_index.write().await;
        for type_set in types.values_mut() {
            type_set.remove(id);
        }

        let mut sizes = self.size_index.write().await;
        for size_set in sizes.values_mut() {
            size_set.remove(id);
        }

        let mut tags = self.tag_index.write().await;
        for tag_set in tags.values_mut() {
            tag_set.remove(id);
        }
    }
}