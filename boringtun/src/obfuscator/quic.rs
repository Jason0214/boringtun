// Copyright (c) 2023 Jiacheng Lu
// SPDX-License-Identifier: BSD-3-Clause

use crate::device::channel::Channel;
use crate::device::Error;

use std::net::SocketAddr;

pub struct QuicObfuscator {

}

impl Channel for QuicObfuscator {
    fn bind(self, port: u16) -> Result<QuicObfuscator, Error> {
        Ok(QuicObfuscator{})
    }

    fn connect(self, dst: &SocketAddr) -> Result<QuicObfuscator, Error> {
        Ok(QuicObfuscator{})
    }

    fn sendto(&self, buf: &[u8], dst: SocketAddr) ->usize {
        0
    }

    fn recvfrom<'a>(&self, buf: &'a mut [u8]) -> Result<(SocketAddr, &'a mut [u8]), Error> {
        Err(Error::IfaceRead(0))
    }

    fn read<'a>(&self, dst: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
        Err(Error::IfaceRead(0))
    }

    fn write(&self, src: &[u8]) -> usize {
        0
    }

    fn shutdown(&self) -> () {

    }

}

impl QuicObfuscator {
    fn new() -> Self {
        // let mut out = [0; MAX_DATAGRAM_SIZE];
        // let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();
        // config.set_max_idle_timeout(5000);
        // config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
        // config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
        // config.set_initial_max_data(10_000_000);
        // config.set_initial_max_stream_data_bidi_local(1_000_000);
        // config.set_initial_max_stream_data_bidi_remote(1_000_000);
        // config.set_initial_max_stream_data_uni(1_000_000);
        // config.set_initial_max_streams_bidi(100);
        // config.set_initial_max_streams_uni(100);
        // config.set_disable_active_migration(true);
        // config.enable_early_data();
        QuicObfuscator {  }
    }
}
