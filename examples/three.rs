use kad::{node_supervisor::{NodeSupervisor, BinaryOptionNodeArgs, NodeInfo}};
use lunatic::{Mailbox, process::StartProcess};
use tracing::info;

#[lunatic::main]
fn main(_: Mailbox<()>) {
    let id = 3;
    let ip = "0.0.0.0".to_string();
    let port = 5551;
    tracing_subscriber::fmt::init();
    let node_args = BinaryOptionNodeArgs{
      id: Some(id),
      k: Some(3),
      bootstrap_node: Some(NodeInfo { ip: ip.clone(), port: 5550, id: 9 }),
      ip,
      port
    };

    let args = ("node".to_owned(), node_args);
    info!("Starting node {:04b} supervisor...", id);
    NodeSupervisor::start_link(args, None);
}
