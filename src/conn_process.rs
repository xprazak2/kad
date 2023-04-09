use std::process::exit;

use lunatic::{process::ProcessRef, abstract_process, net::TcpStream, Mailbox, Process};

use crate::{node_supervisor::NodeProcess, tcp_transport::{KadMsg, TcpTransport}, bucket_list_process::{BucketListProcess, self, BucketListProcessHandler}};

pub struct ConnProcess {
    this: ProcessRef<ConnProcess>,
    bucket_list_process: ProcessRef<BucketListProcess>
    // node_process: ProcessRef<NodeProcess>
}

#[abstract_process(visibility = pub)]
impl ConnProcess {
    #[init]
    fn init(this: ProcessRef<Self>, stream: TcpStream) -> Self {
        let bucket_list_process = ProcessRef::<BucketListProcess>::lookup("bucket_list")
            .expect("could not find bucket list process");

        Process::spawn_link((this.clone(), stream.clone()), |(conn, stream), _: Mailbox<()>| {
            // hoist transport and pass in clone so that we can use it for replies more easily in parent
            let mut transport = TcpTransport::new(stream);

            loop {
                match transport.next() {
                    Ok(msg) => conn.process(msg),
                    Err(err) => panic!("Error occured: {:?}", err)
                }
            }
        });

        Self { this, bucket_list_process }
    }

    #[handle_message]
    fn process(&mut self, msg: KadMsg) {
        match msg {
            KadMsg::Ping{ping_info} => {
                self.bucket_list_process.ping(ping_info.id);
                // send pong
                self.this.exit();
            },
            KadMsg::Pong { ping_info, pong_info } => self.this.exit(),
        }
    }

    #[handle_message]
    fn exit(&mut self) {
        exit(1);
    }
}


