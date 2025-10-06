//! Power/charge, AC and battery

pub(crate) mod battery;
pub(crate) mod charge;

use framework_lib::chromium_ec::CrosEc;

/// Show current power status of battery and AC.
pub(crate) fn info() {
    let ec = CrosEc::new();
    framework_lib::power::get_and_print_power_info(&ec);
}
