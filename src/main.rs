mod subcommands;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: subcommands::item::Command,
}

fn main() {
    let cli = Cli::parse();
    #[allow(unreachable_patterns)]
    match cli.command {
        subcommands::item::Command::Power { .. } => {
            subcommands::item::power::run();
        }
        subcommands::item::Command::Battery { limit } => {
            if let Some(percentage) = limit {
                if percentage < 1 || percentage > 100 {
                    eprintln!("Charge limit must be between 1 and 100");
                    return;
                }
                subcommands::item::battery::set_charge_limit(percentage);
            }
            subcommands::item::battery::get_charge_limit();
        }
        _ => Cli::command().print_help().unwrap(),
    }
}
