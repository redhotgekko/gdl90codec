//! Codec entry point
use crate::{
    error::GDL90Error,
    extended::ExtendedX65SubMessage,
    geometric::OwnershipGeometricAltitude,
    heartbeat::HeartBeat,
    payload::{
        Payload, EXTENDED_X65, HEARTBEAT_ID, OWNERSHIP_GEOMETRIC_ALTITUDE, OWNERSHIP_REPORT_ID,
        TRAFFIC_REPORT_ID,
    },
    report::Report,
};
use std::iter::once;

const CRC16_TABLE: [u16; 256] = create_crc_table();

const fn create_crc_table() -> [u16; 256] {
    let mut table = [0_u16; 256];

    let mut i = 0_u16;
    while i < 256 {
        let mut crc = i << 8;
        let mut bitctr = 0_u16;
        while bitctr < 8 {
            crc = (crc << 1) ^ if (crc & 0x8000) != 0 { 0x1021 } else { 0 };
            bitctr += 1;
        }

        table[i as usize] = crc;
        i += 1
    }

    table
}

#[derive(Debug, PartialEq)]
pub struct GDL90Message {
    message_id: u8,
    data: Vec<u8>,
    checksum: u16,
}

impl GDL90Message {
    /// Encode a [GDL90Message] as bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut result = vec![];
        result.push(0x7e);
        result.push(self.message_id);

        for &b in &self.data {
            if b == 0x7e || b == 0x7d {
                result.push(0x7d);
                result.push(b ^ 0x20);
            } else {
                result.push(b);
            }
        }

        for b in self.checksum.to_le_bytes() {
            if b == 0x7e || b == 0x7d {
                result.push(0x7d);
                result.push(b ^ 0x20);
            } else {
                result.push(b);
            }
        }

        result.push(0x7e);
        result
    }

    /// Decode [Payload] from message data
    pub fn get_payload(&self) -> Result<Payload, GDL90Error> {
        match self.message_id {
            HEARTBEAT_ID => Ok(Payload::HeartBeat(HeartBeat::try_from(&self.data[..])?)),
            OWNERSHIP_REPORT_ID => Ok(Payload::OwnershipReport(Report::try_from(&self.data[..])?)),
            TRAFFIC_REPORT_ID => Ok(Payload::TrafficReport(Report::try_from(&self.data[..])?)),
            OWNERSHIP_GEOMETRIC_ALTITUDE => Ok(Payload::OwnershipGeometricAltitude(
                OwnershipGeometricAltitude::try_from(&self.data[..])?,
            )),
            EXTENDED_X65 => Ok(Payload::ExtendedX65(ExtendedX65SubMessage::try_from(
                &self.data[..],
            )?)),
            _ => Ok(Payload::Unknown(self.message_id, self.data.clone())),
        }
    }
}

/// Create a [GDL90Message] from a u8 slice
pub fn read_message(data: &[u8]) -> Result<GDL90Message, GDL90Error> {
    match data {
        [] => Err(GDL90Error::EmptyData),
        [0x7e, body @ .., 0x07e] => build_message(body),
        [..] => Err(GDL90Error::IncorrectlyFormatted),
    }
}

/// Create a [GDL90Message] for a [Payload]
pub fn create_message(payload: &Payload) -> Result<GDL90Message, GDL90Error> {
    let data = payload.to_bytes()?;

    let message_id = payload.get_message_id();
    let chk_data = once(&message_id).chain(data[..].iter());
    let checksum = checksum(chk_data);
    let data = data.to_vec();

    Ok(GDL90Message {
        message_id,
        data,
        checksum,
    })
}

fn build_message(body: &[u8]) -> Result<GDL90Message, GDL90Error> {
    let data = unescape(body);

    match &data[..] {
        [message_id, payload @ .., checksum1, checksum2] => {
            let crc = [*checksum1, *checksum2];
            let actual_checksum = checksum(data[..data.len() - 2].iter());
            let expected_checksum = u16::from_le_bytes(crc);

            if actual_checksum != expected_checksum {
                Err(GDL90Error::ChecksumMismatch(
                    actual_checksum,
                    expected_checksum,
                ))
            } else {
                Ok(GDL90Message {
                    message_id: *message_id,
                    data: payload.to_vec(),
                    checksum: expected_checksum,
                })
            }
        }
        _ => Err(GDL90Error::IncorrectlyFormatted),
    }
}

fn checksum<'a>(data: impl Iterator<Item = &'a u8>) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        crc = CRC16_TABLE[(crc >> 8) as usize] ^ ((crc << 8) ^ (byte as u16));
    }
    crc
}

fn unescape(data: &[u8]) -> Vec<u8> {
    let mut message_data = vec![];

    let mut is_escape = false;
    for byte in data {
        if is_escape {
            let byte = byte ^ 0x20;
            message_data.push(byte);
            is_escape = false;
        } else if *byte == 0x7d {
            is_escape = true;
        } else {
            message_data.push(*byte);
        }
    }

    message_data
}

#[cfg(test)]
mod test {
    use super::checksum;
    use crate::message::read_message;

    #[test]
    fn test_check_sum() {
        let data = [0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02];

        let expect = u16::from_le_bytes([0xB3, 0x8B]);
        let actual = checksum(data.iter());

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_round_trip() {
        let data = b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xB3\x8B\x7E";

        let message = read_message(data).unwrap();
        let encoded = message.encode();
        assert_eq!(&data[..], &encoded[..]);
    }
}
