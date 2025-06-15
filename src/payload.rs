//! Message payload
use crate::{
    extended::ExtendedX65SubMessage, geometric::OwnershipGeometricAltitude, heartbeat::HeartBeat,
    report::Report,
};
use deku::{DekuContainerWrite, DekuError};

/// Message payload
#[derive(Debug)]
pub enum Payload {
    HeartBeat(HeartBeat),
    OwnershipReport(Report),
    TrafficReport(Report),
    OwnershipGeometricAltitude(OwnershipGeometricAltitude),
    ExtendedX65(ExtendedX65SubMessage),
    Unknown(u8, Vec<u8>),
}

pub(crate) const HEARTBEAT_ID: u8 = 0;
pub(crate) const OWNERSHIP_REPORT_ID: u8 = 10;
pub(crate) const TRAFFIC_REPORT_ID: u8 = 20;
pub(crate) const OWNERSHIP_GEOMETRIC_ALTITUDE: u8 = 11;
pub(crate) const EXTENDED_X65: u8 = 0x65;

impl Payload {
    pub(crate) fn get_message_id(&self) -> u8 {
        match self {
            Payload::HeartBeat(_) => HEARTBEAT_ID,
            Payload::OwnershipReport(_) => OWNERSHIP_REPORT_ID,
            Payload::TrafficReport(_) => TRAFFIC_REPORT_ID,
            Payload::OwnershipGeometricAltitude(_) => OWNERSHIP_GEOMETRIC_ALTITUDE,
            Payload::ExtendedX65(_) => EXTENDED_X65,
            Payload::Unknown(msg_id, _) => *msg_id,
        }
    }

    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, DekuError> {
        match self {
            Payload::HeartBeat(entity) => entity.to_bytes(),
            Payload::OwnershipReport(entity) => entity.to_bytes(),
            Payload::TrafficReport(entity) => entity.to_bytes(),
            Payload::OwnershipGeometricAltitude(entity) => entity.to_bytes(),
            Payload::ExtendedX65(entity) => entity.to_bytes(),
            Payload::Unknown(_, data) => Ok(data.clone()),
        }
    }
}
