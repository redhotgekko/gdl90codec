//! Data type for ownership and traffic messages
use deku::{DekuRead, DekuWrite};
use enum_ordinalize::Ordinalize;
use std::str::{from_utf8, Utf8Error};

#[derive(DekuRead, DekuWrite, Debug, Default)]
#[deku(bit_order = "msb", endian = "big")]
pub struct Report {
    #[deku(bits = "4")]
    traffic_alert_status: u8,

    #[deku(bits = "4")]
    address_type: u8,

    #[deku(bits = "24")]
    pub participant_address: u32,

    #[deku(bits = "24")]
    latitude: u32,

    #[deku(bits = "24")]
    longitude: u32,

    #[deku(bits = "12")]
    altitude: u16,

    #[deku(bits = "2")]
    heading_type: u8,

    #[deku(bits = "1")]
    report_type: u8,

    #[deku(bits = "1")]
    flight_stage: u8,

    #[deku(bits = "4")]
    navigation_integrity_category: u8,

    #[deku(bits = "4")]
    navigation_accuracy_category_for_position: u8,

    #[deku(bits = "12")]
    horizontal_velocity: u16,

    #[deku(bits = "12")]
    vertical_velocity: u16,

    #[deku(bits = "8")]
    track_heading: u8,

    #[deku(bits = "8")]
    emitter_category: u8,

    callsign: [u8; 8],

    #[deku(bits = "4")]
    emergency_priority_code: u8,

    #[deku(bits = "4")]
    reserved: u8,
}

impl Report {
    pub fn get_callsign(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.callsign[..]).to_owned()
    }

    pub fn set_callsign(&mut self, callsign: [u8; 8]) {
        self.callsign = callsign;
    }

    pub fn get_latitude(&self) -> f32 {
        lat_long_u32_to_f32(self.latitude)
    }

    pub fn get_longitude(&self) -> f32 {
        lat_long_u32_to_f32(self.longitude)
    }

    pub fn get_altitude(&self) -> i32 {
        (self.altitude as i32 * 25) - 1000
    }

    pub fn get_traffic_alert_status(&self) -> TrafficAlertStatus {
        TrafficAlertStatus::VARIANTS[self.traffic_alert_status as usize]
    }

    pub fn get_address_type(&self) -> AddressType {
        AddressType::VARIANTS[self.address_type as usize]
    }

    pub fn get_heading_type(&self) -> HeadingType {
        HeadingType::VARIANTS[self.heading_type as usize]
    }

    pub fn get_report_type(&self) -> ReportType {
        ReportType::VARIANTS[self.report_type as usize]
    }

    pub fn get_flight_stage(&self) -> FlightStage {
        FlightStage::VARIANTS[self.flight_stage as usize]
    }

    pub fn get_navigation_integrity_category(&self) -> NIC {
        NIC::VARIANTS[self.navigation_integrity_category as usize]
    }

    pub fn get_navigation_accuracy_category_for_position(&self) -> NACp {
        NACp::VARIANTS[self.navigation_accuracy_category_for_position as usize]
    }

    pub fn get_horizontal_velocity(&self) -> Option<u16> {
        if self.horizontal_velocity >= 0xFFF {
            None
        } else {
            Some(self.horizontal_velocity)
        }
    }

    pub fn get_track_heading(&self) -> f32 {
        (self.track_heading as f32 * 360.) / 256.
    }

    pub fn set_track_heading(&mut self, value: f32) {
        self.track_heading = ((value * 256.) / 360.) as u8;
    }

    pub fn set_horizontal_velocity(&mut self, value: Option<u16>) {
        if let Some(value) = value {
            if value >= 0xFFF {
                self.horizontal_velocity = 0xFFF;
            } else {
                self.horizontal_velocity = value;
            }
        } else {
            self.horizontal_velocity = 0xFFF;
        }
    }

    pub fn get_vertical_velocity(&self) -> Option<i16> {
        if self.vertical_velocity == 0x800 {
            return None;
        }

        let value = if self.vertical_velocity & 0x800 != 0 {
            (self.vertical_velocity as i16) | !0x0FFF // sign-extend to 32 bits
        } else {
            self.vertical_velocity as i16
        };

        Some(value * 64)
    }

    pub fn get_emitter_category(&self) -> EmitterCategory {
        EmitterCategory::VARIANTS[self.emitter_category as usize]
    }

    pub fn get_emergency_priority_code(&self) -> EmergencyPriorityCode {
        EmergencyPriorityCode::VARIANTS[self.emergency_priority_code as usize]
    }

    pub fn set_emergency_priority_code(&mut self, value: EmergencyPriorityCode) {
        self.emergency_priority_code = value.ordinal() as u8;
    }

    pub fn set_emitter_category(&mut self, value: EmitterCategory) {
        self.emitter_category = value.ordinal() as u8;
    }

    pub fn set_navigation_integrity_category(&mut self, value: NIC) {
        self.navigation_integrity_category = value.ordinal() as u8;
    }

    pub fn set_navigation_accuracy_category_for_position(&mut self, value: NACp) {
        self.navigation_accuracy_category_for_position = value.ordinal() as u8;
    }

    pub fn set_vertical_velocity(&mut self, value: Option<i16>) {
        if let Some(value) = value {
            self.vertical_velocity = value as u16 & 0x0fff;
        } else {
            self.vertical_velocity = 0x800;
        }
    }

    pub fn set_flight_stage(&mut self, value: FlightStage) {
        self.flight_stage = value.ordinal() as u8;
    }

    pub fn set_report_type(&mut self, value: ReportType) {
        self.report_type = value.ordinal() as u8;
    }

    pub fn set_heading_type(&mut self, value: HeadingType) {
        self.heading_type = value.ordinal() as u8;
    }

    pub fn set_address_type(&mut self, value: AddressType) {
        self.address_type = value.ordinal() as u8;
    }

    pub fn set_traffic_alert_status(&mut self, value: TrafficAlertStatus) {
        self.traffic_alert_status = value.ordinal() as u8;
    }

    pub fn set_altitude(&mut self, value: i32) {
        self.altitude = ((value + 1000) / 25) as u16;
    }

    pub fn set_latitude(&mut self, value: f32) {
        self.latitude = lat_long_f32_to_u32(value);
    }

    pub fn set_longitude(&mut self, value: f32) {
        self.longitude = lat_long_f32_to_u32(value)
    }
}

fn lat_long_u32_to_f32(value: u32) -> f32 {
    let fraction = if value & 0x800000 != 0 {
        (value as i32) | !0xFFFFFF // sign-extend to 32 bits
    } else {
        value as i32
    };

    ((fraction * 45) as f32) / ((1 << 21) as f32)
}

fn lat_long_f32_to_u32(value: f32) -> u32 {
    let result = (value * ((1 << 21) as f32) / 45.) as i32;
    result as u32 & 0x00ffffff
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum NIC {
    Unknown,
    Lt20_0NM,
    Lt8_0NM,
    Lt4_0NM,
    Lt2_0NM,
    Lt1_0NM,
    Lt0_6NM,
    Lt0_2nm,
    Lt0_1NM,
    HPLlt75mAndVPLlt112m,
    HPLlt25mAndVPLlt37_5m,
    HPLlt7_5mAndVPLlt11m,
    Unused1,
    Unused2,
    Unused3,
    Unused4,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum NACp {
    Unknown,
    Lt10_0NM,
    Lt4_0NM,
    Lt2_0NM,
    Lt1_0NM,
    Lt0_5NM,
    Lt0_3NM,
    Lt0_1NM,
    Lt0_05NM,
    HFOMlt30mAndVFOMlt45m,
    HFOMlt10mAndVFOMlt15m,
    HFOMlt3mandVFOMlt4m,
    Unused1,
    Unused2,
    Unused3,
    Unused4,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum EmergencyPriorityCode {
    NoEmergency,
    GeneralEmergency,
    MedicalEmergency,
    MinimumFuel,
    NoCommunication,
    UnlawfulInterference,
    DownedAircraft,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum EmitterCategory {
    NoAircraftTypeInformation,
    Light,
    Small,
    Large,
    HighVortexLarge,
    Heavy,
    HighlyManeuverable,
    Rotorcraft,
    Unassigned1,
    GliderSailplane,
    LighterThanAir,
    ParachutistSkyDiver,
    UltraLightHangGliderParaglider,
    Unassigned2,
    UnmannedAerialVehicle,
    SpaceTransatmosphericVehicle,
    Unassigned3,
    SurfaceVehicleEmergencyVehicle,
    SurfaceVehicleServiceVehicle,
    PointObstacle,
    ClusterObstacle,
    LineObstacle,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Reserved11,
    Reserved12,
    Reserved13,
    Reserved14,
    Reserved15,
    Reserved16,
    Reserved17,
    Reserved18,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum FlightStage {
    OnGround,
    Airborne,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum ReportType {
    ReportIsUpdated,
    ReportIsExtrapolated,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum HeadingType {
    NotValid,
    TrueTrackAngle,
    HeadingMagnetic,
    HeadingTrue,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum AddressType {
    ADSBWithICAOAddress,
    ADSBWithSelfAssignedAddress,
    TISBWithICAOAddress,
    TISBWithTrackFileID,
    SurfaceVehicle,
    GroundStationBeacon,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
}

#[derive(Ordinalize, Debug, PartialEq, Clone, Copy)]
pub enum TrafficAlertStatus {
    NoAlert,
    TrafficAlert,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Reserved11,
    Reserved12,
    Reserved13,
    Reserved14,
}

#[cfg(test)]
mod test {
    use deku::DekuContainerWrite;

    use crate::report::{
        AddressType, EmergencyPriorityCode, EmitterCategory, FlightStage, HeadingType, NACp,
        ReportType, NIC,
    };

    use super::{lat_long_f32_to_u32, lat_long_u32_to_f32, Report, TrafficAlertStatus};

    #[test]
    fn test_report() {
        let data: Vec<u8> = vec![
            0x00, 0x40, 0xaa, 0xbb, 0x24, 0x8e, 0x4a, 0xff, 0xb1, 0x6e, 0x16, 0x79, 0x89, 0x10,
            0x4f, 0xff, 0x12, 0x05, 0x54, 0x45, 0x53, 0x54, 0x20, 0x20, 0x20, 0x20, 0x06,
        ];

        let report = Report::try_from(&data[..]).unwrap();

        assert_eq!(report.get_callsign().unwrap(), "TEST    ".to_owned());
        assert_eq!(
            report.get_traffic_alert_status(),
            TrafficAlertStatus::NoAlert
        );
        assert_eq!(report.get_address_type(), AddressType::ADSBWithICAOAddress);
        assert_eq!(report.participant_address, 0x40AABB); // Fake address
        assert_eq!(report.get_latitude(), 51.406617164612);
        assert_eq!(report.get_longitude(), -0.43159961700439);
        assert_eq!(report.get_altitude(), 7975);
        assert_eq!(report.get_flight_stage(), FlightStage::Airborne);
        assert_eq!(report.get_report_type(), ReportType::ReportIsUpdated);
        assert_eq!(report.navigation_integrity_category, 8);
        assert_eq!(report.navigation_accuracy_category_for_position, 9);
        assert_eq!(report.get_horizontal_velocity(), Some(260));
        assert_eq!(report.get_vertical_velocity(), Some(-64));
        assert_eq!(report.get_track_heading(), 25.3125);
        assert_eq!(report.get_navigation_integrity_category(), NIC::Lt0_1NM);
        assert_eq!(
            report.get_navigation_accuracy_category_for_position(),
            NACp::HFOMlt30mAndVFOMlt45m
        );
        assert_eq!(report.get_emitter_category(), EmitterCategory::Heavy);
        assert_eq!(
            report.get_emergency_priority_code(),
            EmergencyPriorityCode::NoEmergency
        );

        let encoded = report.to_bytes().unwrap();
        assert_eq!(&data[..], &encoded[..]);
    }

    #[test]
    fn test_lat_long_convert1() {
        let start_value = 2395560;
        let latlong = lat_long_u32_to_f32(start_value);
        let encoded = lat_long_f32_to_u32(latlong);

        let expect_latlong = 51.40314_f32;

        assert_eq!(expect_latlong, latlong);
        assert_eq!(start_value, encoded);
    }

    #[test]
    fn test_lat_long_convert2() {
        let start_value = 16756978;
        let latlong = lat_long_u32_to_f32(start_value);
        let encoded = lat_long_f32_to_u32(latlong);

        let expect_latlong = -0.43426037_f32;

        assert_eq!(expect_latlong, latlong);
        assert_eq!(start_value, encoded);
    }

    #[test]
    fn test_traffic_alert_status() {
        let mut report = Report::default();
        report.set_traffic_alert_status(TrafficAlertStatus::Reserved8);
        assert_eq!(
            TrafficAlertStatus::Reserved8,
            report.get_traffic_alert_status()
        );
    }

    #[test]
    fn test_address_type() {
        let mut report = Report::default();
        report.set_address_type(AddressType::Reserved8);
        assert_eq!(AddressType::Reserved8, report.get_address_type());
    }

    #[test]
    fn test_heading_type() {
        let mut report = Report::default();
        report.set_heading_type(HeadingType::TrueTrackAngle);
        assert_eq!(HeadingType::TrueTrackAngle, report.get_heading_type());
    }

    #[test]
    fn test_report_type() {
        let mut report = Report::default();
        report.set_report_type(ReportType::ReportIsExtrapolated);
        assert_eq!(ReportType::ReportIsExtrapolated, report.get_report_type());
    }

    #[test]
    fn test_flight_stage() {
        let mut report = Report::default();
        report.set_flight_stage(FlightStage::Airborne);
        assert_eq!(FlightStage::Airborne, report.get_flight_stage());
    }
}
