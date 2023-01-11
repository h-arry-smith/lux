use std::net::{ToSocketAddrs, UdpSocket};

use crate::universe::Universe;

use self::sacn::{DataPacket, MAX_PACKET_LENGTH};

pub mod sacn;

const UUID: &str = "ebe9c992-f874-43b5-a303-91498162881b";

// TODO: Find a common trait for output types to provide same interace for
//       different protocols, when we have more than just SACN :)

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NetworkState {
    Uninitialized,
    Bound,
    Connected,
}

pub struct NetworkOutput {
    socket: Option<UdpSocket>,
    protocol: SACN,
    pub state: NetworkState,
}

impl NetworkOutput {
    pub fn new() -> Self {
        Self {
            socket: None,
            protocol: SACN::new(
                "Candela Test".to_string(),
                uuid::Uuid::parse_str(UUID).unwrap(),
            ),
            state: NetworkState::Uninitialized,
        }
    }

    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        let socket = UdpSocket::bind(addr)?;
        self.socket = Some(socket);
        self.state = NetworkState::Bound;
        Ok(())
    }

    pub fn connect(&mut self, addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        self.socket.as_mut().unwrap().connect(addr)?;
        self.state = NetworkState::Connected;
        Ok(())
    }

    pub fn send_data(&mut self, universe: &Universe) -> Result<(), std::io::Error> {
        if let Some(socket) = &self.socket {
            match self.protocol.send_universe(universe, socket) {
                Ok(_) => Ok(()),
                Err(err) => {
                    self.state = NetworkState::Bound;
                    Err(err)
                }
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "no socket",
            ))
        }
    }
}

impl Default for NetworkOutput {
    fn default() -> Self {
        Self::new()
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

    pub fn send_universe(
        &mut self,
        universe: &Universe,
        socket: &UdpSocket,
    ) -> Result<usize, std::io::Error> {
        let mut buf = [0; MAX_PACKET_LENGTH];
        self.pack_data_packet(&mut buf, universe);

        self.seq_number = self.seq_number.wrapping_add(1);

        // TODO: Propogate this error much more nicely and deal with it in the
        //       common network output, reseting state to an unconnected state
        //       if necessary.
        socket.send(&buf)
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
