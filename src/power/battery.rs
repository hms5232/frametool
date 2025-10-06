use framework_lib::chromium_ec::CrosEc;

/// Get the battery charge limit
pub(crate) fn get_max_percentage() {
    let ec = CrosEc::new();
    let charge_limit = ec.get_charge_limit();
    match charge_limit {
        Ok(limitation) => println!("Charge limit is {}%", limitation.1),
        Err(e) => eprintln!("Error occurred when try to get charge limitation: {:?}", e),
    }
}

/// Set the battery charge limit
pub(crate) fn set_max_percentage(max: u8) {
    let ec = CrosEc::new();
    let result = ec.set_charge_limit(0, max);
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("Error occurred when try to set charge limitation: {:?}", e),
    }
}
