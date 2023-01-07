// Specification: https://tsp.esta.org/tsp/documents/docs/ANSI_E1-31-2018.pdf

use byteorder::{ByteOrder, NetworkEndian};

use crate::universe::Universe;

pub const MAX_PACKET_LENGTH: usize = 638;
pub const ACN_SDT_MULTICAST_PORT: u16 = 5568;

pub struct DataPacket<'a> {
    root_layer: RootLayer<'a>,
    framing_layer: DataPacketFramingLayer,
    dmp_layer: DMPLayer,
}

impl<'a> DataPacket<'a> {
    pub fn new(
        universe: &Universe,
        source_name: &str,
        priority: Option<u8>,
        sync_address: u16,
        seq_number: u8,
        options: u8,
        cid: &'a [u8; 16],
    ) -> Self {
        let dmp_layer = DMPLayer::new(universe.bytes());
        let framing_layer = DataPacketFramingLayer::new(
            dmp_layer.len() as u16,
            source_name,
            priority,
            sync_address,
            seq_number,
            options,
            universe.universe_number() as u16,
        );
        let root_layer = RootLayer::new(framing_layer.len() as u16, VECTOR_ROOT_E131_DATA, cid);

        Self {
            root_layer,
            framing_layer,
            dmp_layer,
        }
    }

    pub fn pack(&self, buf: &mut [u8]) {
        self.root_layer.pack(buf);
        self.framing_layer.pack(buf);
        self.dmp_layer.pack(buf);
    }
}

// 5 use of the ACN Root Layer Protocol
// 5.1 Preamble Size - Sources shall set the Preamble Size to 0x0010
const ROOT_LAYER_PREAMBLE_SIZE: u16 = 0x0010;
// 5.2 Post-amble Size - Sources shall set the Post-amble Size to 0x0000
const ROOT_LAYER_POSTAMBLE_SIZE: u16 = 0x0000;
// 5.3 ACN Packet Identifier
const ACN_PACKET_IDENTIFIER: [u8; 12] = [
    0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00,
];
// 5.4 Flags & Length
const ROOT_LAYER_FLAGS: u16 = 0x7000;
// 5.5 Vector
// Sources shall set the Root Layer's Vector to VECTOR_ROOT_E131_DATA if the
// packet contains E1.31 Data, or to VECTOR_ROOT_E131_EXTENDED if the packet is
// for Universe Discovery or for Synchronization.
const VECTOR_ROOT_E131_DATA: u32 = 0x0000_0004;
const _VECTOR_ROOT_E131_EXTENDED: u32 = 0x0000_0008;

struct RootLayer<'a> {
    preamble_size: u16,
    postamble_size: u16,
    acn_packet_identifier: [u8; 12],
    flags_and_length: u16,
    vector: u32,
    cid: &'a [u8; 16],
}

impl<'a> RootLayer<'a> {
    fn new(length: u16, vector: u32, cid: &'a [u8; 16]) -> Self {
        let length = length + 22;

        Self {
            preamble_size: ROOT_LAYER_PREAMBLE_SIZE,
            postamble_size: ROOT_LAYER_POSTAMBLE_SIZE,
            acn_packet_identifier: ACN_PACKET_IDENTIFIER,
            flags_and_length: ROOT_LAYER_FLAGS | length & 0x0fff,
            vector,
            cid,
        }
    }

    fn pack(&self, buf: &mut [u8]) {
        NetworkEndian::write_u16(&mut buf[0..2], self.preamble_size);
        NetworkEndian::write_u16(&mut buf[2..4], self.postamble_size);
        buf[4..16].copy_from_slice(&self.acn_packet_identifier);
        NetworkEndian::write_u16(&mut buf[16..18], self.flags_and_length);
        NetworkEndian::write_u32(&mut buf[18..22], self.vector);
        buf[22..38].copy_from_slice(self.cid);
    }
}

// 6.1 Flags & Length
const DATA_FRAMING_FLAGS: u16 = 0x7000;

// 6.2 E1.31 Data Packet Framing Layer
// 6.2.1 E1.31 Data Packet: Vector - Sources sending an E1.31 Data Packet shall set
// the E1.31 Layer's Vector to VECTOR_E131_DATA_PACKET
const VECTOR_E131_DATA_PACKET: u32 = 0x0000_0002;

// 6.2.6 Data Packet: Options
// This bit-oriented field is used to encode optional flags that control how
// the packet is used.

// Preview_Data: Bit 7
// This bit, when set to 1, indicates that the data in this packet is intended
// for use in visualization or media server preview applications and shall not
// be used to generate live output.
pub const OPT_PREVIEW_DATA: u8 = 0b1000_0000;

// Stream_Terminated: Bit 6
// This bit is intended to allow E1.31 sources to terminate transmission of a
// stream or of universe synchronization without waiting for a timeout to occur,
// and to indicate to receivers that such termination is not a fault condition.
pub const OPT_STREAM_TERMINATED: u8 = 0b0100_0000;

// Force_Synchronization: Bit 5
// This bit indicates whether to lock or revert to an unsynchronized state when
// synchronization is lost
pub const OPT_FORCE_SYNC: u8 = 0b0010_0000;

struct DataPacketFramingLayer {
    flags_and_length: u16,
    vector: u32,
    source_name: String,
    priority: u8,
    sync_address: u16,
    seq_number: u8,
    options: u8,
    universe: u16,
}

impl DataPacketFramingLayer {
    fn new(
        length: u16,
        source_name: &str,
        priority: Option<u8>,
        sync_address: u16,
        seq_number: u8,
        options: u8,
        universe: u16,
    ) -> Self {
        let length = length + 77;

        // 6.2.2 E1.31 Data Packet: Source Name
        // A user-assigned name provided by the source of the packet for use in
        // displaying the identity of a source to a user. There is no mechanism,
        // other than user configuration, to ensure uniqueness of this name. The
        // source name shall be null-terminated.
        let source_name = String::from_utf8(source_name.bytes().take(63).collect())
            .unwrap_or_else(|_| "bad name".to_string());
        let mut source_name = format!("{: <63}", source_name);
        source_name.push(0x00 as char);

        // 6.2.3 E1.31 Data Packet: Priority
        // Sources that do not support variable priority shall transmit a
        // priority of 100. No priority outside the range of 0 to 200 shall be
        // transmitted on the network.
        let priority = match priority {
            Some(priority) => priority.clamp(0, 200),
            None => 100,
        };

        // 6.2.7 E1.31 Data Packet: Universe
        // The Universe is a 16-bit field that defines the universe number of
        // the data carried in the packet. Universe values shall be limited to
        // the range 1 to 63999.
        let universe = universe.clamp(1, 63999);

        Self {
            flags_and_length: DATA_FRAMING_FLAGS | length & 0x0fff,
            vector: VECTOR_E131_DATA_PACKET,
            source_name,
            priority,
            sync_address,
            seq_number,
            options,
            universe,
        }
    }

    fn pack(&self, buf: &mut [u8]) {
        NetworkEndian::write_u16(&mut buf[38..40], self.flags_and_length);
        NetworkEndian::write_u32(&mut buf[40..44], self.vector);
        buf[44..108].copy_from_slice(self.source_name.as_bytes());
        buf[108] = self.priority;
        NetworkEndian::write_u16(&mut buf[109..111], self.sync_address);
        buf[111] = self.seq_number;
        buf[112] = self.options;
        NetworkEndian::write_u16(&mut buf[113..115], self.universe);
    }

    fn len(&self) -> usize {
        (self.flags_and_length & 0x0fff) as usize
    }
}

// TODO: 6.3 E1.31 Synchronization Packet Framing Layer

// 7 DMP Layer Protocol
// 7.1 DMP Layer: Flags & Length
const DMP_LAYER_FLAGS: u16 = 0x7000;
// 7.2 DMP Layer: Vector
// The DMP Layer's Vector shall be set to VECTOR_DMP_SET_PROPERTY, which
// indicates a DMP Set Property message by sources.
const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;
// 7.3 Address Type and Data Type
// Sources shall set the DMP Layer's Address Type and Data Type to 0xa1
const DMP_ADDRESS_DATA_TYPE: u8 = 0xa1;
// 7.4 First Property Address
// Sources shall set the DMP Layer's First Property Address to 0x0000
const FIRST_PROPERTY_ADDRESS: u16 = 0x0000;
// 7.5 Address Increment
// Sources shall set the DMP Layer's Address Increment to 0x0001.
const ADDRESS_INCREMENT: u16 = 0x0001;
// 7.7 Property Values
// The first octet of the property values field shall be the DMX512-A [DMX] START Code
const DMX_START_CODE: u8 = 0x00;

struct DMPLayer {
    flags_and_length: u16,
    vector: u8,
    address_and_data_type: u8,
    first_property_address: u16,
    address_increment: u16,
    property_value_count: u16,
    property_values: [u8; 512],
}

impl DMPLayer {
    fn new(property_values: [u8; 512]) -> Self {
        // The DMP Layer PDU length is computed starting with octet 115 and
        // continuing through the last property value provided in the DMP PDU
        // (octet 637 for a full payload).
        let length = 11 + property_values.len();

        Self {
            flags_and_length: DMP_LAYER_FLAGS | length as u16 & 0x0fff,
            vector: VECTOR_DMP_SET_PROPERTY,
            address_and_data_type: DMP_ADDRESS_DATA_TYPE,
            first_property_address: FIRST_PROPERTY_ADDRESS,
            address_increment: ADDRESS_INCREMENT,
            // 7.6 Property Value Count
            // The DMP Layer's Property Value Count is used to encode the number
            // of DMX512-A [DMX] Slots (including the START Code slot).
            property_value_count: (property_values.len() + 1) as u16,
            property_values,
        }
    }

    fn pack(&self, buf: &mut [u8]) {
        NetworkEndian::write_u16(&mut buf[115..117], self.flags_and_length);
        buf[117] = self.vector;
        buf[118] = self.address_and_data_type;
        NetworkEndian::write_u16(&mut buf[119..121], self.first_property_address);
        NetworkEndian::write_u16(&mut buf[121..123], self.address_increment);
        NetworkEndian::write_u16(&mut buf[123..125], self.property_value_count);
        buf[125] = DMX_START_CODE;
        buf[126..(126 + self.property_values.len())].copy_from_slice(&self.property_values)
    }

    fn len(&self) -> usize {
        11 + self.property_values.len()
    }
}
