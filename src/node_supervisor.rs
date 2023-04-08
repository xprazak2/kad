use lunatic::{supervisor::Supervisor, process::{ProcessRef, StartProcess}, abstract_process, Process};

use crate::bucket_list_process::BucketListProcess;
use std::convert::From;
use serde::{Serialize, Deserialize};

use uuid::Uuid;
use sha3::{Digest, Sha3_512};
use rand::Rng;

pub enum IdKind {
    Binary(BinaryOptionNodeArgs)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryOptionNodeArgs {
    pub id: Option<u64>,
    pub k: Option<u64>
}

pub struct NodeArgs {
    pub id: u64,
    pub k: u64
}

impl From<BinaryOptionNodeArgs> for NodeArgs {
    fn from(item: BinaryOptionNodeArgs) -> Self {
        let id = item.id.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen()
        });

        let k = item.id.unwrap_or(3);

        Self { id, k }
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
    bucket_list: ProcessRef<BucketListProcess>
}

#[abstract_process(visibility = pub)]
impl NodeProcess {
    #[init]
    fn init(this: ProcessRef<Self>, args: BinaryOptionNodeArgs) -> Self {

        let bucket_list = BucketListProcess::start_link((), None);

        Self{ bucket_list }
    }
}