use deku::prelude::*;

#[derive(DekuRead, DekuWrite, Debug, Default, PartialEq)]
#[deku(bit_order = "msb", endian = "big")]
pub struct Initialization {
    #[deku(bits = "1")]
    reserved1: bool,
    #[deku(bits = "1")]
    pub audio_test: bool,
    #[deku(bits = "1")]
    reserved2: bool,
    #[deku(bits = "1")]
    reserved3: bool,
    #[deku(bits = "1")]
    reserved4: bool,
    #[deku(bits = "1")]
    reserved5: bool,
    #[deku(bits = "1")]
    pub audio_inhibit: bool,
    #[deku(bits = "1")]
    pub cdti_ok: bool,

    #[deku(bits = "1")]
    reserved6: bool,
    #[deku(bits = "1")]
    reserved7: bool,
    #[deku(bits = "1")]
    reserved8: bool,
    #[deku(bits = "1")]
    reserved9: bool,
    #[deku(bits = "1")]
    reserved10: bool,
    #[deku(bits = "1")]
    reserved11: bool,
    #[deku(bits = "1")]
    pub csa_audio_disable: bool,
    #[deku(bits = "1")]
    pub csa_disable: bool,
}

#[cfg(test)]
mod test {
    use crate::initialization::Initialization;

    #[test]
    pub fn test() {
        let data = b"\x41\x01";

        let init = Initialization::try_from(&data[..]).unwrap();

        assert!(!init.audio_inhibit);
        assert!(init.audio_test);
        assert!(init.cdti_ok);
        assert!(!init.csa_audio_disable);
        assert!(init.csa_disable);
    }
}
