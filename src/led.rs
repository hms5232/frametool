//! Fingerprint/power button LED setting

use framework_lib::chromium_ec::commands::FpLedBrightnessLevel;
use framework_lib::chromium_ec::{CrosEc, EcResult};

pub fn get_and_print_info() {
    let ec = CrosEc::new();
    match ec.get_fp_led_level() {
        Ok((brightness, level)) => {
            println!("FP LED level: {:?}", level);
            println!("FP LED brightness: {}", brightness);
        }
        Err(e) => {
            eprintln!("Failed to get FP LED level: {:?}", e);
        }
    }
}

/// Set the fingerprint/power button LED level
///
/// # Arguments
///
/// * `level` - The LED level, see `framework_lib::commandline::FpLedBrightnessLevel` for details.
pub fn set_level(level: framework_lib::commandline::FpBrightnessArg) -> EcResult<()> {
    let ec = CrosEc::new();
    ec.set_fp_led_level(FpLedBrightnessLevel::from(level))
}

/// Set the fingerprint/power button LED brightness
///
/// # Arguments
///
/// * `percentage` - The brightness percentage, range from 1 to 100.
pub fn set_brightness(percentage: u8) -> EcResult<()> {
    let ec = CrosEc::new();
    match ec.set_fp_led_percentage(percentage) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to set FP LED brightness: {:?}", e);
            Err(e)
        }
    }
}
