//! Ownership geometric altitude message
use deku::prelude::*;

#[derive(DekuRead, DekuWrite, Debug, Default, PartialEq)]
#[deku(bit_order = "msb", endian = "big")]
pub struct OwnershipGeometricAltitude {
    ownship_geo_altitude: u16,

    #[deku(bits = "1")]
    vertical_warning_indicator: bool,

    #[deku(bits = "15")]
    vertical_figure_of_merit: u16,
}

impl OwnershipGeometricAltitude {
    /// Ownship Geo Altitude
    pub fn get_ownship_geo_altitude(&self) -> i32 {
        (self.ownship_geo_altitude * 5) as i32
    }

    /// Ownship Geo Altitude
    pub fn set_ownship_geo_altitude(&mut self, value: u32) {
        self.ownship_geo_altitude = (value / 5) as u16;
    }

    /// Vertical Metrics (Vertical Warning indicator)
    pub fn get_vertical_warning_indicator(&self) -> bool {
        self.vertical_warning_indicator
    }

    /// Vertical Metrics (Vertical Warning indicator)
    pub fn set_vertical_warning_indicator(&mut self, value: bool) {
        self.vertical_warning_indicator = value;
    }

    /// Vertical Metrics (Vertical Figure of Merit, in meters)
    pub fn get_vertical_figure_of_merit(&self) -> Option<u16> {
        if self.vertical_figure_of_merit == 0x7FFF {
            None
        } else {
            Some(self.vertical_figure_of_merit)
        }
    }

    /// Vertical Metrics (Vertical Figure of Merit, in meters)
    pub fn set_vertical_figure_of_merit(&mut self, value: Option<u16>) {
        if let Some(value) = value {
            self.vertical_figure_of_merit = value;
        } else {
            self.vertical_figure_of_merit = 0x7FFF;
        }
    }
}

#[cfg(test)]
mod test {
    use deku::DekuContainerWrite;

    use super::OwnershipGeometricAltitude;

    #[test]
    fn test_encode_decode() {
        let data = [0x00, 0x22, 0x00, 0x75];
        let geo = OwnershipGeometricAltitude::try_from(&data[..]).unwrap();

        let mut new_geo = OwnershipGeometricAltitude::default();
        new_geo.set_ownship_geo_altitude(170);
        new_geo.set_vertical_figure_of_merit(Some(117));

        let encode = new_geo.to_bytes().unwrap();

        assert_eq!(geo, new_geo);
        assert_eq!(&data[..], &encode[..]);
    }
}
