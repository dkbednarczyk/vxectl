mod debounce;
mod device;
mod info;
mod performance;
mod sensor;
mod sleep;

use clap::{builder::PossibleValuesParser, value_parser, Parser, Subcommand};
use device::Device;

#[derive(Parser)]
#[command(name = "vxectl")]
#[command(about = "Control your VXE gaming mouse from the command line")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set device parameters
    #[command(arg_required_else_help = true)]
    Set {
        /// DPI stage to enable
        #[arg(short = 'd', long, value_parser = value_parser!(u8).range(1..=6))]
        dpi_stage: Option<u8>,

        /// Polling rate in Hz
        #[arg(short = 'p', long, value_parser = PossibleValuesParser::new(["125", "250", "500", "1000", "2000", "4000", "8000"]))]
        polling_rate: Option<String>,

        /// Sensor setting to enable
        #[arg(short = 'x', long, value_parser = PossibleValuesParser::new(["basic", "competitive", "max"]))]
        sensor_setting: Option<String>,

        /// Debounce time in milliseconds
        #[arg(short = 'b', long, value_parser = PossibleValuesParser::new(["0", "1", "2", "4", "8", "15", "20"]))]
        debounce: Option<String>,

        /// Sleep timeout (inactivity before sleep)
        #[arg(short = 's', long, value_parser = PossibleValuesParser::new(["30s", "1m", "2m", "3m", "5m", "20m", "25m", "30m"]))]
        sleep: Option<String>,
    },

    /// Get device info
    #[clap(subcommand)]
    Info(Info),
}

#[derive(Subcommand)]
enum Info {
    // Get battery status
    Battery,
}

fn main() {
    let cli = Cli::parse();

    let device = match Device::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Set {
            dpi_stage,
            polling_rate,
            sensor_setting,
            debounce,
            sleep,
        } => {
            // Validate polling rate for wired devices
            if let Some(rate_str) = &polling_rate {
                let rate: u16 = rate_str.parse().unwrap();
                if device.is_wired() && rate > 1000 {
                    eprintln!("Wired mouse only supports up to 1000 Hz polling rate.");
                    return;
                }
            }

            // Send sensor setting first if present
            if let Some(setting_str) = sensor_setting {
                if let Err(e) = sensor::apply_setting(&device, &setting_str) {
                    eprintln!("{}", e);
                }
            }

            // Send debounce setting if present
            if let Some(debounce_str) = debounce {
                if let Err(e) = debounce::apply_setting(&device, &debounce_str) {
                    eprintln!("{}", e);
                }
            }

            // Send combined DPI + polling rate packet
            if let Err(e) = performance::apply_settings(&device, dpi_stage, polling_rate.as_deref())
            {
                eprintln!("{}", e);
            }

            // Send sleep timeout setting if present
            if let Some(time) = sleep {
                if let Err(e) = sleep::apply_setting(&device, &time) {
                    eprintln!("{}", e);
                }
            }
        }
        Commands::Info(cmd) => match cmd {
            Info::Battery => {
                if let Err(e) = info::get_battery(&device) {
                    eprintln!("Error retrieving battery info: {}", e);
                }
            }
        },
    }
}
