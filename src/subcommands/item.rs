use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Get the charger and battery status, equal to `framework_tool --power`
    Power {
        //
    },
    /// Get (without argument) or set (provide 1-100) the battery charge limit
    Battery {
        /// Set the battery charge limit (1-100)
        limit: Option<u8>,
    },
}

pub(crate) mod power {
    use framework_lib::chromium_ec::CrosEc;
    use framework_lib::power::get_and_print_power_info;

    pub(crate) fn run() {
        let ec = CrosEc::new();
        get_and_print_power_info(&ec);
    }
}

pub(crate) mod battery {
    use framework_lib::chromium_ec::CrosEc;

    pub(crate) fn get_charge_limit() {
        let ec = CrosEc::new();
        let charge_limit = ec.get_charge_limit();
        match charge_limit {
            Ok(limitation) => println!("Charge limit is {}", limitation.1),
            Err(e) => eprintln!("Error occurred when try to get charge limitation: {:?}", e),
        }
    }

    pub(crate) fn set_charge_limit(limitation: u8) {
        let ec = CrosEc::new();
        let result = ec.set_charge_limit(0, limitation);
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("Error occurred when try to get charge limitation: {:?}", e),
        }
    }
}
