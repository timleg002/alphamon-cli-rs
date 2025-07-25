# alphamon-cli-rs

CLI interface for Alpha Outback UPSes written in Rust. Based on the [alphamon-rs](https://github.com/timleg002/alphamon-rs) library. Works as an alternative to the proprietary Alphamon software by the manufacturer. 

Currently supports only querying the UPSes, not performing tests or changing settings.

Tested on Alphamon Continuity Plus models 3000 and 1000 via USB and serial communication interface.

## Usage

```
Usage: alphamon-cli-rs [OPTIONS] --type <INTERFACE_TYPE> <COMMAND>

Commands:
  query    Queries the UPS for various info
  setting  Configures the UPS settings
  help     Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>            UPS device path (valid for serial or usb-hid interface type)
  -v, --vid-pid <VID_PID>      UPS device VID:PID (valid only for usb-hid interface type)
  -t, --type <INTERFACE_TYPE>  UPS interface type [possible values: usb-hid, serial]
  -h, --help                   Print help
  -V, --version                Print version
```