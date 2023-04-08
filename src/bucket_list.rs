use chrono::{DateTime, Utc};

pub struct PeerNode {
    ip: String,
    port: u64,
    id: u64,
    last_seen: DateTime<Utc>
}

pub struct BucketList {
    k: u64,
    root: Bucket
}

impl BucketList {
    pub fn new(k: u64) -> Self {
        Self { k, root: Bucket::BucketLeaf(vec![]) }
    }
}

pub enum Bucket {
    BucketNode((Box<Bucket>, Box<Bucket>)),
    BucketLeaf(Vec<PeerNode>)
}
