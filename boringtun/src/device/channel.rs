use super::Error;
use super::UDPSocket;

use std::net::SocketAddr;
use std::sync::Arc;

pub trait Channel {
    fn init(udp: Arc<UDPSocket>) -> ();

    fn sendto(&self, buf: &[u8], dst: SocketAddr) -> usize;

    fn recvfrom<'a>(&self, buf: &'a mut [u8]) -> Result<(SocketAddr, &'a mut [u8]), Error>;

    fn read<'a>(&self, dst: &'a mut [u8]) -> Result<&'a mut [u8], Error>;

    fn write(&self, src: &[u8]) -> usize;

    fn close(&self) -> ();
}
