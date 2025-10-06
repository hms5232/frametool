//! AC charge setting

use framework_lib::chromium_ec::CrosEc;

/// Set the AC charge rate
///
/// # Arguments
///
/// * `rate` - The charge rate in C. "1C" means charging from 0 to 100% takes 1 hour.
/// * `battery_soc` - The condition of battery SoC in percentage, optional.
///
/// # Example
///
/// Set charge rate limit 0.8C only if battery is >80% charged.
///
/// ```rs
/// set_rate_limit(0.8, 80);
/// ```
pub(crate) fn set_rate_limit(
    rate: f32,
    battery_soc: Option<f32>,
) -> Result<(), framework_lib::chromium_ec::EcError> {
    let ec = CrosEc::new();
    ec.set_charge_rate_limit(rate, battery_soc)
}
