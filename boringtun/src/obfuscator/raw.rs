// Copyright (c) 2023 Jiacheng Lu
// SPDX-License-Identifier: BSD-3-Clause

use crate::device::channel::Channel;
use crate::device::udp::UDPSocket;
use crate::device::Error;

use std::net::SocketAddr;

pub struct NoneObfuscator {
    udp_socket: UDPSocket,
}

impl Channel for NoneObfuscator {
    fn init(udp: std::sync::Arc<crate::device::udp::UDPSocket>) -> () {}

    fn sendto(&self, buf: &[u8], dst: SocketAddr) -> usize {
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

    fn close(&self) -> () {}
}
