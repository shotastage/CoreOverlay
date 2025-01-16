// Implementation of RPC protocol

use crate::NodeId;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum RpcMessage {
    Ping,
    Store { key: Vec<u8>, value: Vec<u8> },
    FindNode { target: NodeId },
    FindValue { key: Vec<u8> },
}

pub struct Rpc {
    socket: UdpSocket,
}

impl Rpc {
    pub fn new() -> Self {
        // UDPソケットの初期化
        todo!()
    }

    pub async fn send(&self, msg: RpcMessage, addr: SocketAddr) {
        // メッセージのシリアライズと送信
        todo!()
    }

    pub async fn receive(&self) -> (RpcMessage, SocketAddr) {
        // メッセージの受信とデシリアライズ
        todo!()
    }
}
