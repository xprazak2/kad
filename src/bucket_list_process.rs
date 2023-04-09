use lunatic::abstract_process;
use lunatic::{process::{ProcessRef}};

use crate::bucket_list::{BucketList, Bucket};
use crate::node_supervisor::NodeInfo;


pub struct BucketListProcess {
  bucket_list: BucketList
}

#[abstract_process(visibility = pub)]
impl BucketListProcess {
    #[init]
    fn init(_this: ProcessRef<Self>, args: (u64, u64)) -> Self {
        Self { bucket_list: BucketList::new(args.0, args.1) }
    }

    #[handle_message]
    fn ping(&mut self, peer_id: u64) {
        let res = self.bucket_list.ping(peer_id);
        // reply to whoever pinged
    }

}
