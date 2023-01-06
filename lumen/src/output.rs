use crate::universe::Universe;

use self::sacn::{DataPacket, MAX_PACKET_LENGTH};

pub mod sacn;

// TODO: Find a common trait for output types to provide same interace for
//       different protocols, when we have more than just SACN :)

// TODO: Implementing a simple unicast for universe output, but should also
//       support universe multicasting

pub struct SACNOutputter {
    source_name: String,
    cid: uuid::Uuid,
    seq_number: u8,
}

impl SACNOutputter {
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
