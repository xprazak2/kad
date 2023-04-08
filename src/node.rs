use crate::bucket_list::BucketList;

pub struct Node {
    pub id: u64,
    pub id_size: u64,
}

impl Node {
    fn new(id: u64, id_size: u64, k: u64) -> Self {
        return Self { id: id % id_size, id_size }
    }

    fn id_bin(&self) -> String {
        format!("{:width$b}", self.id, width = self.id_size as usize)
    }


}