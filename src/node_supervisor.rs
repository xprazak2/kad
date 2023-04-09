use lunatic::{supervisor::Supervisor, process::{ProcessRef, StartProcess}, abstract_process, Process};
use tracing::info;

use crate::{bucket_list_process::BucketListProcess, node::Node};
use std::convert::From;
use serde::{Serialize, Deserialize};

use uuid::Uuid;
use sha3::{Digest, Sha3_512};
use rand::Rng;

pub enum IdKind {
    Binary(BinaryOptionNodeArgs)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeInfo {
    pub ip: String,
    pub port: u64,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryOptionNodeArgs {
    pub id: Option<u64>,
    pub port: u64,
    pub ip: String,
    pub k: Option<u64>,
    pub bootstrap_node: Option<NodeInfo>
}

pub struct NodeArgs {
    pub k: u64,
    pub node_info: NodeInfo
}

impl From<BinaryOptionNodeArgs> for NodeArgs {
    fn from(item: BinaryOptionNodeArgs) -> Self {
        let id = item.id.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen()
        });

        let k = item.id.unwrap_or(3);

        Self { node_info: NodeInfo { ip: item.ip, port: item.port, id }, k }
    }
}


pub struct NodeSupervisor {}

impl Supervisor for NodeSupervisor {
    type Arg = (String, BinaryOptionNodeArgs);
    type Children = NodeProcess;

    fn init(config: &mut lunatic::supervisor::SupervisorConfig<Self>, arg: Self::Arg) {
        // config.children_args(((), arg))
        config.children_args((arg.1, Some(arg.0)))
    }
}

pub struct NodeProcess {
    bucket_list: ProcessRef<BucketListProcess>,
    node: Node
}

#[abstract_process(visibility = pub)]
impl NodeProcess {
    #[init]
    fn init(this: ProcessRef<Self>, args: BinaryOptionNodeArgs) -> Self {
        let node_args: NodeArgs = args.into();
        let bucket_list = BucketListProcess::start_link((node_args.k, node_args.node_info.id), Some("bucket_list"));
        info!("Starting node process...");
        Self{ bucket_list, node: Node::new(node_args.node_info) }
    }
}