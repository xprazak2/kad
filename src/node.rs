use crate::{bucket_list::BucketList, node_supervisor::NodeInfo};

pub struct Node {
    pub id: u64,
    pub port: u64,
    pub ip: String
}


impl Node {
    pub fn new(node_info: NodeInfo) -> Self {
        return Self { id: node_info.id, ip: node_info.ip, port: node_info.port }
    }

    fn id_bin(&self) -> String {
        format!("{:b}", self.id)
    }
}