mod led;
mod power;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Power/charge, AC and battery information/setting.
    /// Run without subcommand, show the charger and battery status (equal to `framework_tool --power`).
    /// To see usage of subcommands, run with `-h`.
    Power {
        #[command(subcommand)]
        command: Option<PowerCommand>,
    },
    /// Get (without argument) or set (provide 1-100) the battery charge limit
    Battery {
        /// Set the battery charge limit (1-100)
        #[clap(value_parser=clap::value_parser!(u8).range(1..=100))]
        limit: Option<u8>,
    },
    /// Power button/fingerprint LED information/setting.
    /// Run without subcommand, show the LED status (equal to `framework_tool --fp-led-level`).
    /// To see usage of subcommands, run with `-h`.
    Led {
        #[command(subcommand)]
        command: Option<LedCommand>,
    },
}

/// Subcommands under `power`.
#[derive(Subcommand, Debug)]
enum PowerCommand {
    /// Get the charger and battery status, equal to `framework_tool --power`
    Info,
    /// Charge rate, current setting
    Charge {
        #[command(subcommand)]
        command: PowerChargeCommand,
    },
}

/// Subcommands under `power charge`.
#[derive(Subcommand, Debug)]
enum PowerChargeCommand {
    /// Set the AC charge rate
    Rate {
        /// The charge rate in C. "1C" means charging from 0 to 100% takes 1 hour.
        rate: f32,
        /// The condition of battery SoC in percentage.
        soc: Option<f32>,
    },
}

/// Subcommands under `led`.
#[derive(Subcommand, Debug)]
enum LedCommand {
    /// Get (without argument) or set (provide level) the LED level
    Level {
        level: Option<framework_lib::commandline::FpBrightnessArg>,
    },
    /// Get (without argument) or set (provide percentage) the LED brightness
    Brightness {
        #[clap(value_parser=clap::value_parser!(u8).range(1..=100))]
        percentage: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();
    #[allow(unreachable_patterns)]
    match cli.command {
        Command::Power { command } => match command {
            Some(PowerCommand::Info) => power::info(),
            Some(PowerCommand::Charge { command }) => match command {
                PowerChargeCommand::Rate { rate, soc } => {
                    let _ = power::charge::set_rate_limit(rate, soc);
                }
            },
            None => {
                power::info();
                println!("\n\n>>> To see usage of subcommands under power, run with `-h`.")
            }
        },
        Command::Battery { limit } => {
            if let Some(percentage) = limit {
                power::battery::set_max_percentage(percentage);
            }
            power::battery::get_max_percentage();
        }
        Command::Led { command } => match command {
            Some(LedCommand::Level { level }) => {
                if let Some(level) = level {
                    let _ = led::set_level(level);
                }
                led::get_and_print_info();
            }
            Some(LedCommand::Brightness { percentage }) => {
                if let Some(percentage) = percentage {
                    let _ = led::set_brightness(percentage);
                }
                led::get_and_print_info();
            }
            None => {
                led::get_and_print_info();
                println!("\n\n>>> To see usage of subcommands under led, run with `-h`.")
            }
        },
        _ => Cli::command().print_help().unwrap(),
    }
}
