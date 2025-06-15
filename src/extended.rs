//! Extended specification for message ID 0x65
//!
//! Reference: <https://www.foreflight.com/connect/spec/>
use deku::prelude::*;
use std::str::{from_utf8, Utf8Error};

const ID_SUB_ID: u8 = 0;
const AHRS_SUB_ID: u8 = 1;

/// ID sub message (ID 0x00)
#[derive(DekuRead, DekuWrite, Debug, Default, PartialEq)]
#[deku(bit_order = "msb", endian = "big")]
pub struct IDMessage {
    sub_id: u8,
    pub version: u8,
    pub device_serial_number: u64,
    device_name: [u8; 8],
    device_long_name: [u8; 16],

    #[deku(bits = "1")]
    geometric_altitude_datum: u8,

    #[deku(bits = "2")]
    internet_policy: u8,

    #[deku(bits = "29")]
    unused: u32,
}

impl IDMessage {
    pub fn get_device_name(&self) -> Result<String, Utf8Error> {
        let mut end_idx = self.device_name.len();
        for (idx, b) in self.device_name.iter().enumerate() {
            if *b == 0 {
                end_idx = idx;
                break;
            }
        }
        from_utf8(&self.device_name[0..end_idx]).map(|s| s.to_owned())
    }

    pub fn get_device_long_name(&self) -> Result<String, Utf8Error> {
        let mut end_idx = self.device_name.len();
        for (idx, b) in self.device_name.iter().enumerate() {
            if *b == 0 {
                end_idx = idx;
                break;
            }
        }
        from_utf8(&self.device_long_name[0..end_idx]).map(|s| s.to_owned())
    }
}

/// Attitude and heading reference system (AHRS) sub message (ID 0x01)
#[derive(DekuRead, DekuWrite, Debug, Default, PartialEq)]
#[deku(bit_order = "msb", endian = "big")]
pub struct AHRS {
    sub_id: u8,
    roll: i16,
    pitch: i16,

    #[deku(bits = "1")]
    heading_type: u8,

    #[deku(bits = "15")]
    heading: u16,

    indicated_airspeed: u16,

    true_airspeed: u16,
}

/// Sub message container enum
#[derive(Debug)]
pub enum ExtendedX65SubMessage {
    AHRS(AHRS),
    IDMessage(IDMessage),
}

impl ExtendedX65SubMessage {
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, DekuError> {
        match self {
            ExtendedX65SubMessage::IDMessage(payload) => {
                Ok(set_sub_id(payload.to_bytes()?, ID_SUB_ID))
            }
            ExtendedX65SubMessage::AHRS(payload) => {
                Ok(set_sub_id(payload.to_bytes()?, AHRS_SUB_ID))
            }
        }
    }
}

impl TryFrom<&[u8]> for ExtendedX65SubMessage {
    type Error = DekuError;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        if let Some(submsgid) = input.first() {
            match *submsgid {
                ID_SUB_ID => parse_id_message(input),
                AHRS_SUB_ID => parse_ahrs_message(input),
                id => Err(DekuError::Parse(
                    format!("Unknown sub message id {id}").into(),
                ))?,
            }
        } else {
            Err(DekuError::Parse(
                "Expect at least one byte containing the sub message id".into(),
            ))?
        }
    }
}

fn set_sub_id(mut payload: Vec<u8>, message_id: u8) -> Vec<u8> {
    payload[0] = message_id;
    payload
}

fn parse_id_message(input: &[u8]) -> Result<ExtendedX65SubMessage, DekuError> {
    let message = IDMessage::try_from(input)?;
    Ok(ExtendedX65SubMessage::IDMessage(message))
}

fn parse_ahrs_message(input: &[u8]) -> Result<ExtendedX65SubMessage, DekuError> {
    let message = AHRS::try_from(input)?;
    Ok(ExtendedX65SubMessage::AHRS(message))
}

#[cfg(test)]
mod test {
    use super::ExtendedX65SubMessage;

    #[test]
    fn test_parse_id() {
        let data = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x03, 0x02, 0x01, 0x53, 0x6B, 0x79, 0x45,
            0x63, 0x68, 0x6F, 0x00, 0x53, 0x6B, 0x79, 0x45, 0x63, 0x68, 0x6F, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        if let Ok(ExtendedX65SubMessage::IDMessage(ref payload)) =
            ExtendedX65SubMessage::try_from(&data[..])
        {
            assert_eq!(payload.device_serial_number, 0x04_03_02_01); // Fake serial number
            assert_eq!(payload.get_device_name().unwrap(), "SkyEcho");
            assert_eq!(payload.get_device_long_name().unwrap(), "SkyEcho");
        } else {
            panic!()
        }
    }
}
