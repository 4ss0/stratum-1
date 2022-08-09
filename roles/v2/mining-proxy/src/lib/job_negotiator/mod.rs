use roles_logic_sv2::{utils::Mutex, parsers::TemplateDistribution, handlers::SendTo_, parsers::PoolMessages};
use codec_sv2::StandardEitherFrame;
use std::sync::Arc;
use tokio::net::TcpListener;
use async_channel::{Receiver, Sender};
use codec_sv2::{HandshakeRole, Initiator};
use tokio::task;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use network_helpers::noise_connection_tokio::Connection;


pub type SendTo = SendTo_<TemplateDistribution<'static>, ()>;
pub type EitherFrame = StandardEitherFrame<PoolMessages<'static>>;

pub struct  JobNegotiator{
    sender: Sender<StandardEitherFrame<PoolMessages<'static>>>,
    receiver: Receiver<StandardEitherFrame<PoolMessages<'static>>>
}

impl JobNegotiator{
    pub async fn new(address: SocketAddr, authority_public_key: [u8; 32]) {
        let stream = TcpStream::connect(address).await.unwrap();
        let initiator = Initiator::from_raw_k(authority_public_key).unwrap();
        let (receiver, sender): (Receiver<EitherFrame>, Sender<EitherFrame>) =
        Connection::new(stream,HandshakeRole::Initiator(initiator)).await;
        task::spawn(async { loop{tokio::time::sleep(std::time::Duration::from_millis(10)).await } });
    }
}
