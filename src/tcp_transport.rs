use lunatic::net::TcpStream;
use serde::{Serialize, Deserialize};
use crate::{tcp_transport::KadMsg::Ping, node_supervisor::NodeInfo};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KadMsg {
  Ping{ping_info: NodeInfo},
  Pong{ping_info: NodeInfo, pong_info: NodeInfo}
}

#[derive(Error, Debug, Clone)]
pub enum KadTransportError {
    #[error("unknown transport error")]
    Unknown,

    #[error("deserialization error")]
    Deserialization
}

pub struct TcpTransport {
  stream: TcpStream
}

impl TcpTransport {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn next(&mut self) -> Result<KadMsg, KadTransportError> {
        // get underlying message from ciborium error
        let st = ciborium::de::from_reader::<KadMsg, TcpStream>(self.stream.clone())
            .map_err(|err| KadTransportError::Deserialization);
        st
    }
}