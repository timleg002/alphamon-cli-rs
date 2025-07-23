extern crate lovely_env_logger;
#[macro_use] extern crate log;

mod query;

use alphamon_rs::device::cplus;
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

/// CLI interface for Alpha Outback UPSes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// UPS device path
    #[arg(short = 'p', long)]
    path: String,
    /// UPS interface type
    #[arg(short = 't', long = "type")]
    interface_type: InterfaceType,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Queries the UPS for various info
    Query {
        #[command(subcommand)]
        command: QueryCommands,
    },
    /// Configures the UPS settings
    Setting {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum InterfaceType {
    UsbHid,
    Serial
}

#[derive(Subcommand, Debug)]
enum QueryCommands {
    /// Prints the status of the UPS (input voltage, fault voltage, output voltage, load percentage, etc.)
    Status,
    /// Prints extra info about the UPS (output frequency, battery voltage, UPS load in watts, error code, etc.)
    ExtraInfo,
    /// Prints the length of time the UPS is able to run without external power given the current load
    Autonomy,
    /// Prints info about inverter status and UPS alarm state
    Alarm,
    /// Prints info about the UPS rating
    Rating,
    /// Prints info about the UPS manufacturer, model and version
    Info,
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {}

#[tokio::main]
async fn main() -> Result<()> {
    lovely_env_logger::init(lovely_env_logger::Config {
        short_levels: true,
        ..Default::default()
    });

    let args = Args::parse();

    let mut dev: Box<dyn cplus::CPlusInterface> = match args.interface_type {
        InterfaceType::UsbHid => Box::new(cplus::CPlusHidInterface::connect_with_path(args.path)?),
        InterfaceType::Serial => Box::new(cplus::CPlusSerialInterface::connect(&args.path)?),
    };

    match args.command {
        Commands::Query { command } => {
            query::print_formatted_query(&mut dev, &command)?;
        }
        Commands::Setting { command } => {
            error!("Unsupported command {command:?}");
        },
    };

    Ok(())
}
