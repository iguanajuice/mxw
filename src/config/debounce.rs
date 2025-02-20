use hidapi::HidDevice;
use crate::lib::getstatus::check_sleep;
use super::DEFAULT_PROFILE;

pub fn set(device: &HidDevice, profile: Option<u8>, ms: u8) {
    check_sleep(device);

    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[6] = 0x08;

    bfr[7] = profile_id;
    bfr[8] = ms;

    device.send_feature_report(&bfr).unwrap();
}
