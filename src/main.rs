use lunatic::{Mailbox, process::StartProcess};
use clap::{Arg, Command, Parser};

use crate::node_supervisor::{NodeSupervisor, BinaryOptionNodeArgs};
use crate::cli::Cli;


mod node_supervisor;
mod node;
mod bucket_list;
mod bucket_list_process;
mod cli;


#[lunatic::main]
fn main(_: Mailbox<()>) {
    let args = ("node".to_owned(), BinaryOptionNodeArgs{ id: Some(2), k: Some(3) });
    NodeSupervisor::start_link(args, None);
}
