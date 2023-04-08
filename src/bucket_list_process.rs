use lunatic::abstract_process;
use lunatic::{process::{ProcessRef}};


pub struct BucketListProcess {}

#[abstract_process(visibility = pub)]
impl BucketListProcess {
    #[init]
    fn init(_this: ProcessRef<Self>, _: ()) -> Self {
        Self {}
    }
}
