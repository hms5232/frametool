//! Power/charge, AC and battery

pub(crate) mod battery;

use framework_lib::chromium_ec::CrosEc;
use framework_lib::power::get_and_print_power_info;

/// Show current power status of battery and AC.
pub(crate) fn info() {
    let ec = CrosEc::new();
    get_and_print_power_info(&ec);
}
