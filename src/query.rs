use crate::QueryCommands;
use alphamon_rs::{
    device::cplus,
    model::cplus::{
        AlarmInquiryResponse, AutonomyResponse, ExtraPowerInfoResponse, StatusInquiryResponse,
        UPSInformation, UPSRating, UPSStatus,
    },
};
use anyhow::Result;
use humanize_duration::prelude::DurationExt;
use indoc::printdoc;

pub(crate) fn print_formatted_query(
    dev: &mut Box<dyn cplus::CPlusInterface>,
    command: &QueryCommands,
) -> Result<()> {
    match command {
        QueryCommands::Status => {
            let StatusInquiryResponse {
                input_voltage,
                input_fault_voltage,
                output_voltage,
                output_load_percentage,
                input_frequency,
                battery_capacity,
                battery_capacity_parameter: _,
                temperature,
                ups_status:
                    UPSStatus {
                        utility_fail,
                        battery_low,
                        bypass_or_transformer_active,
                        battery_abnormal,
                        offline,
                        test_in_progress,
                        shutdown_active,
                        beeper_on,
                    },
            } = dev.query_ups_status()?;

            let b_or_t = if offline {
                "Boost/buck converter active"
            } else {
                "Bypass mode"
            };

            printdoc!(
                r##"
                    Input voltage: {input_voltage} V
                    Input fault voltage: {input_fault_voltage} V
                    Input frequency: {input_frequency} Hz

                    Output voltage: {output_voltage} V
                    Output load percentage: {output_load_percentage} %
                    
                    Battery capacity: {battery_capacity} %

                    Temperature: {temperature} °C

                    Utility fail: {utility_fail}
                    Battery low: {battery_low}
                    {b_or_t}: {bypass_or_transformer_active}
                    Battery abnormal: {battery_abnormal}
                    Offline: {offline}
                    Test in progress: {test_in_progress}
                    Shutdown active: {shutdown_active}
                    Beeper on: {beeper_on}
                "##
            );
        }
        QueryCommands::ExtraInfo => {
            let ExtraPowerInfoResponse {
                ups_output_freq,
                battery_voltage,
                battery_cut_voltage,
                ups_wattage,
                error_code,
                load_current,
            } = dev.query_extra_power_info()?;

            printdoc!(
                r##"
                    Output frequency: {ups_output_freq} Hz
                    UPS load: {ups_wattage} W
                    Load current: {load_current} A

                    Battery voltage: {battery_voltage} V
                    Battery cut voltage: {battery_cut_voltage} V

                    Error code: {error_code}
                "##
            );
        }
        QueryCommands::Autonomy => {
            let AutonomyResponse { time } = dev.query_ups_autonomy()?;

            printdoc!("Time left: {}", time.human(humanize_duration::Truncate::Second));
        }
        QueryCommands::Alarm => {
            let AlarmInquiryResponse {
                inverter_on,
                ups_alarm_on,
            } = dev.query_alarm()?;

            printdoc!(
                r##"
                    Inverter on: {inverter_on}
                    UPS alarm on: {ups_alarm_on}
                "##
            );
        }
        QueryCommands::Rating => {
            let UPSRating {
                output_rating_voltage,
                output_rating_current,
                battery_voltage,
                output_rating_frequency,
            } = dev.query_ups_rating()?;

            printdoc!(
                r##"
                    Output rating voltage: {output_rating_voltage} V
                    Output rating current: {output_rating_current} A
                    Output rating frequency: {output_rating_frequency} Hz

                    Battery voltage: {battery_voltage} V
                "##
            );
        }
        QueryCommands::Info => {
            let UPSInformation {
                manufacturer_name,
                model,
                version,
            } = dev.query_ups_info()?;

            printdoc!(
                r##"
                    Manufacturer: {manufacturer_name}
                    Model: {model}
                    Version: {version}
                "##
            );
        }
    };
    Ok(())
}
