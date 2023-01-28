use super::Error;
use std::net::SocketAddr;

pub trait Channel {
    fn bind(self, port: u16) -> Result<Self, Error>
    where
        Self: Sized;

    fn connect(self, dst: &SocketAddr) -> Result<Self, Error>
    where
        Self: Sized;

    fn sendto(&self, buf: &[u8], dst: SocketAddr) -> usize;

    fn recvfrom<'a>(&self, buf: &'a mut [u8]) -> Result<(SocketAddr, &'a mut [u8]), Error>;

    fn read<'a>(&self, dst: &'a mut [u8]) -> Result<&'a mut [u8], Error>;

    fn write(&self, src: &[u8]) -> usize;

    fn shutdown(&self) -> ();
}
