use kad::{node_supervisor::{NodeSupervisor, BinaryOptionNodeArgs, NodeInfo}, conn_process::ConnProcess};
use lunatic::{Mailbox, process::StartProcess, net::TcpListener, ProcessConfig};
use tracing::info;

#[lunatic::main]
fn main(_: Mailbox<()>) {
    tracing_subscriber::fmt::init();

    let id = 9;
    let ip = "0.0.0.0".to_string();
    let port = 5550;

    let node_args = BinaryOptionNodeArgs{
        id: Some(id),
        k: Some(3),
        bootstrap_node: None,
        port,
        ip: ip.clone(),
    };

    let args = ("node".to_owned(), node_args);
    info!("Starting node {:04b} supervisor...", id);
    NodeSupervisor::start_link(args, None);

    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(address).expect("could not bind the TCP port");

    let mut conn_conf = ProcessConfig::new().expect("could not create connection process config");
    conn_conf.set_can_spawn_processes(true);

    while let Ok((stream, _)) = listener.accept() {
        ConnProcess::start_config(stream, None, &conn_conf);
    }
}
