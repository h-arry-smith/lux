use std::{
    net::{ToSocketAddrs, UdpSocket},
    time::{Duration, Instant},
};

use crate::universe::Universe;

use self::sacn::{DataPacket, MAX_PACKET_LENGTH};

pub mod sacn;

// TODO: Find a common trait for output types to provide same interace for
//       different protocols, when we have more than just SACN :)

pub enum NetworkState {
    Uninitialized,
    Bound,
    Connected,
}

pub struct NetworkOutput {
    socket: Option<UdpSocket>,
    state: NetworkState,
    last_connection_attempt: Instant,
}

impl NetworkOutput {
    pub fn new() -> Self {
        Self {
            socket: None,
            state: NetworkState::Uninitialized,
            last_connection_attempt: Instant::now(),
        }
    }

    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        let socket = UdpSocket::bind(addr)?;
        self.socket = Some(socket);
        Ok(())
    }

    pub fn try_connect(&mut self, addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        if self.last_connection_attempt.elapsed() > Duration::new(0, 1_000_000_000 / 2) {
            self.connect(addr)
        } else {
            self.last_connection_attempt = Instant::now();
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "not ready to reconnect",
            ))
        }
    }

    fn connect(&mut self, addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        self.socket.as_mut().unwrap().connect(addr)?;
        self.state = NetworkState::Connected;
        Ok(())
    }
}

// TODO: Implementing a simple unicast for universe output, but should also
//       support universe multicasting

pub struct SACN {
    source_name: String,
    cid: uuid::Uuid,
    seq_number: u8,
}

impl SACN {
    pub fn new(source_name: String, cid: uuid::Uuid) -> Self {
        Self {
            source_name,
            cid,
            seq_number: 0,
        }
    }

    pub fn send_universe(&mut self, universe: &Universe) {
        let mut buf = [0; MAX_PACKET_LENGTH];
        self.pack_data_packet(&mut buf, universe);

        self.seq_number += 1;

        // TODO: Send a universe via an established connection to a server
    }

    pub fn pack_data_packet(&mut self, buf: &mut [u8], universe: &Universe) {
        // TODO: Support the various options in the struct below
        let packet = DataPacket::new(
            universe,
            &self.source_name,
            None,
            0,
            self.seq_number,
            0x0,
            self.cid.as_bytes(),
        );

        packet.pack(buf);
    }
}
