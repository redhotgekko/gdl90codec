//! Heartbeat message
use deku::prelude::*;

#[derive(DekuRead, DekuWrite, Debug, Default, PartialEq)]
#[deku(bit_order = "msb", endian = "big")]
pub struct HeartBeat {
    #[deku(bits = "1")]
    pub gps_pos_valid: bool,

    #[deku(bits = "1")]
    pub maint_req: bool,

    #[deku(bits = "1")]
    pub ident: bool,

    #[deku(bits = "1")]
    pub addr_type: bool,

    #[deku(bits = "1")]
    pub gps_batt_low: bool,

    #[deku(bits = "1")]
    pub ratcs: bool,

    #[deku(bits = "1")]
    reserved1: bool,

    #[deku(bits = "1")]
    pub uat_initialized: bool,

    #[deku(bits = "1")]
    time_stamp_msb: bool,

    #[deku(bits = "1")]
    pub csa_requested: bool,

    #[deku(bits = "1")]
    pub csa_not_available: bool,

    #[deku(bits = "1")]
    reserved2: bool,

    #[deku(bits = "1")]
    reserved3: bool,

    #[deku(bits = "1")]
    reserved4: bool,

    #[deku(bits = "1")]
    reserved5: bool,

    #[deku(bits = "1")]
    pub utc_ok: bool,

    #[deku(bits = "16")]
    time_stamp: u16,

    #[deku(bits = "16")]
    pub message_counts: u16,
}

impl HeartBeat {
    pub fn get_time_stamp(&self) -> u32 {
        if self.time_stamp_msb {
            1_u32 << 17 | (self.time_stamp as u32)
        } else {
            self.time_stamp.swap_bytes() as u32
        }
    }

    pub fn set_time_stamp(&mut self, value: u32) {
        self.time_stamp_msb = (value & 1_u32 << 17) != 0;
        self.time_stamp = (value as u16).swap_bytes();
    }
}

#[cfg(test)]
mod test {
    use deku::DekuContainerWrite;

    use super::HeartBeat;

    #[test]
    fn test_encode_decode1() {
        let data = [0x01, 0x00, 0xf7, 0xd1, 0x00, 0x00];
        let mut heartbeat = HeartBeat::try_from(&data[..]).unwrap();

        assert_eq!(53751, heartbeat.get_time_stamp());

        heartbeat.set_time_stamp(53751);

        let encode = heartbeat.to_bytes().unwrap();
        assert_eq!(&encode[..], &data[..]);
    }

    #[test]
    fn test_encode_decode2() {
        let data = [0x01, 0x00, 0xf7, 0xd1, 0x00, 0x00];
        let mut heartbeat = HeartBeat::default();

        heartbeat.uat_initialized = true;
        heartbeat.set_time_stamp(53751);

        assert_eq!(53751, heartbeat.get_time_stamp());

        let encoded = heartbeat.to_bytes().unwrap();
        assert_eq!(&data[..], &encoded[..]);
    }

    #[test]
    fn test_encode_decode3() {
        let data = [0x81, 0x80, 0x7d, 0x12, 0x00, 0x00];
        let heartbeat = HeartBeat::try_from(&data[..]).unwrap();

        let mut new_heartbeat = HeartBeat::default();

        new_heartbeat.gps_pos_valid = true;
        new_heartbeat.uat_initialized = true;
        new_heartbeat.time_stamp_msb = true;
        new_heartbeat.time_stamp = 32018;

        let new_data = new_heartbeat.to_bytes().unwrap();

        assert_eq!(heartbeat, new_heartbeat);
        assert_eq!(&data[..], &new_data[..]);
    }

    #[test]
    fn test_timestamp() {
        let mut heartbeat = HeartBeat::default();
        heartbeat.set_time_stamp(53236);

        assert_eq!(53236, heartbeat.get_time_stamp());
    }
}
