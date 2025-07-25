extern crate lovely_env_logger;
#[macro_use]
extern crate log;

mod query;

use alphamon_rs::device::cplus;
use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};

/// CLI interface for Alpha Outback UPSes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// UPS device path (valid for serial or usb-hid interface type)
    #[arg(short = 'p', long)]
    path: Option<String>,
    /// UPS device VID:PID (valid only for usb-hid interface type)
    #[arg(short = 'v', long)]
    vid_pid: Option<String>,
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

#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum InterfaceType {
    UsbHid,
    Serial,
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

    assert!(
        args.vid_pid.is_none() || args.interface_type != InterfaceType::Serial,
        "Serial port path cannot be specified by VID/PID"
    );

    assert_eq!(
        args.vid_pid.is_some(),
        args.path.is_none(),
        "Specify only VID/PID or device path"
    );

    assert!(
        args.vid_pid.as_ref().map(|vid_pid| vid_pid.len() == 9) != Some(false),
        "VID/PID must have a length of 9 characters (format: `ffff:ffff` or `ffff/ffff`)"
    );

    let mut dev: Box<dyn cplus::CPlusInterface> = match args.interface_type {
        InterfaceType::Serial => Box::new(cplus::CPlusSerialInterface::connect(
            &args.path.unwrap(),
        )?),
        InterfaceType::UsbHid => Box::new(
            if let Some(vid_pid) = &args.vid_pid {
                let (vid, pid) = (&vid_pid[0..4], &vid_pid[5..9]);
                let (vid, pid) = (u16::from_str_radix(vid, 16)?, u16::from_str_radix(pid, 16)?);
                cplus::CPlusHidInterface::connect_with_vid_pid(vid, pid)?
            } else if let Some(device_path) = &args.path {
                cplus::CPlusHidInterface::connect_with_path(device_path.to_string())?
            } else {
                bail!("USB HID device must be either initialized with a device path or a VID/PID")
            }
        ),
    };

    match args.command {
        Commands::Query { command } => {
            query::print_formatted_query(&mut dev, &command)?;
        }
        Commands::Setting { command } => {
            error!("Unsupported command {command:?}");
        }
    };

    Ok(())
}
