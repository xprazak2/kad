use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


pub struct PeerNode {
    ip: String,
    port: u64,
    id: u64,
    last_seen: DateTime<Utc>
}

pub struct BucketList {
    k: u64,
    id: u64,
    root: Bucket
}

impl BucketList {
    pub fn new(k: u64, id: u64) -> Self {
        Self { k, id, root: Bucket::BucketLeaf(vec![]) }
    }

    pub fn ping(&mut self, peer_id: u64) {
        // try to insert peer into bucket
    }
}

pub enum Bucket {
    BucketNode((Box<Bucket>, Box<Bucket>)),
    BucketLeaf(Vec<PeerNode>)
}
