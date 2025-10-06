mod power;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Get the charger and battery status, equal to `framework_tool --power`
    Power {
        //
    },
    /// Get (without argument) or set (provide 1-100) the battery charge limit
    Battery {
        /// Set the battery charge limit (1-100)
        #[clap(value_parser=clap::value_parser!(u8).range(1..=100))]
        limit: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();
    #[allow(unreachable_patterns)]
    match cli.command {
        Command::Power { .. } => {
            power::info();
        }
        Command::Battery { limit } => {
            if let Some(percentage) = limit {
                power::battery::set_max_percentage(percentage);
            }
            power::battery::get_max_percentage();
        }
        _ => Cli::command().print_help().unwrap(),
    }
}
