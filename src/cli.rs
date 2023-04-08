use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional node id
    #[arg(short, long)]
    pub id: Option<i64>,
}
