use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;

use clap::{builder::PossibleValuesParser, value_parser, Parser, Subcommand};

use madr_lib::{
    battery::Battery,
    debounce::{self, Debounce},
    device::Device,
    dpi,
    performance::{self, Performance, PollingRate},
    sensor::{self, Mode, Sensor},
    sleep,
};

#[derive(Parser)]
#[command(name = "madrctl")]
#[command(about = "Control your VXE MAD R series gaming mouse from the command line")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure device settings
    #[clap(subcommand)]
    Set(Set),

    /// Change dpi settings
    #[clap(subcommand)]
    Dpi(Dpi),

    /// Get device info
    #[clap(subcommand)]
    Info(Info),
}

#[derive(Subcommand)]
enum Set {
    /// Set debounce time
    Debounce {
        /// Debounce time in milliseconds
        #[arg(value_parser = PossibleValuesParser::new(["0", "1", "2", "4", "8", "15", "20"]))]
        time: String,
    },
    /// Set sleep timeout
    Sleep {
        /// Sleep timeout (inactivity before sleep)
        #[arg(value_parser = PossibleValuesParser::new(["30s", "1m", "2m", "3m", "5m", "20m", "25m", "30m"]))]
        timeout: String,
    },
    /// Set active DPI stage
    DpiStage {
        /// DPI stage to set active (1-8)
        #[arg(value_parser = value_parser!(u8).range(1..=8))]
        stage: u8,
    },
    /// Set polling rate
    PollingRate {
        /// Polling rate in Hz
        #[arg(value_parser = PossibleValuesParser::new(["125", "250", "500", "1000", "2000", "4000", "8000"]))]
        rate: String,
    },
    /// Set sensor preset
    Sensor {
        /// Preset to apply
        #[arg(value_parser = PossibleValuesParser::new(["basic", "competitive", "max"]))]
        preset: String,
    },
}

#[derive(Subcommand)]
enum Info {
    /// Get battery status
    Battery,
    /// Get sensor settings
    Sensor,
}

#[derive(Subcommand)]
enum Dpi {
    /// Change DPI settings for a specific stage
    ModifyStage {
        /// DPI stage to change (1-8)
        #[arg(short, long, value_parser = value_parser!(u8).range(1..=8))]
        stage: u8,
        /// X DPI value
        #[arg(short, long, value_parser = value_parser!(u16).range(50..=16000))]
        x_dpi: Option<u16>,
        /// Y DPI value, if not specified, X DPI will be used
        #[arg(short, long, value_parser = value_parser!(u16).range(50..=16000))]
        y_dpi: Option<u16>,
        /// RGB color in 255,255,255 format, if not specified, color will not be changed
        #[arg(short, long)]
        rgb: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let device = Device::open()?;

    match cli.command {
        Commands::Set(cmd) => match cmd {
            Set::Debounce { time } => {
                let time_val: u8 = time.parse()?;

                if let 0..=2 = time_val {
                    println!("warning: low debounce values are not recommended")
                }

                debounce::apply_setting(&device, Debounce::try_from(time_val)?)?;
            }
            Set::Sleep { timeout } => {
                let duration = match timeout.as_str() {
                    "30s" => Duration::from_secs(30),
                    "1m" => Duration::from_secs(60),
                    "2m" => Duration::from_secs(120),
                    "3m" => Duration::from_secs(180),
                    "5m" => Duration::from_secs(300),
                    "20m" => Duration::from_secs(1200),
                    "25m" => Duration::from_secs(1500),
                    "30m" => Duration::from_secs(1800),
                    _ => return Err(anyhow!("invalid timeout value: {}", timeout)),
                };

                sleep::apply_setting(&device, duration)?;
            }
            Set::DpiStage { stage } => {
                let settings = Performance::read(&device)?;

                performance::apply_setting(
                    &device,
                    &Performance::new(stage, settings.polling_rate()),
                )?;
            }
            Set::PollingRate { rate } => {
                let r: u16 = rate.parse().unwrap();

                // Validate polling rate for wired devices
                if device.is_wired() && r > 1000 {
                    return Err(anyhow!(
                        "Wired mouse only supports up to 1000 Hz polling rate."
                    ));
                }

                let new_rate = PollingRate::try_from(r)?;
                let settings = Performance::read(&device)?;
                performance::apply_setting(
                    &device,
                    &Performance::new(settings.dpi_stage(), new_rate),
                )?;
            }
            Set::Sensor { preset } => {
                let preset: Mode = preset.parse()?;

                sensor::apply_setting(&device, preset)?;
            }
        },
        Commands::Dpi(cmd) => match cmd {
            Dpi::ModifyStage {
                stage,
                x_dpi,
                y_dpi,
                rgb,
            } => {
                dpi::apply_dpi_setting(&device, stage, x_dpi, y_dpi, rgb.as_deref())?;
            }
        },
        Commands::Info(cmd) => match cmd {
            Info::Battery => {
                let b = Battery::read(&device)?;

                let colored_percentage = match b.percentage() {
                    0..=20 => format!("{}", b.percentage()).red(),
                    21..=50 => format!("{}", b.percentage()).yellow(),
                    _ => format!("{}", b.percentage()).green(),
                };

                println!(
                    "{colored_percentage}% | {:.2}V | {}",
                    (b.voltage() as f32 / 1000.0),
                    if b.is_charging() {
                        "Charging".green()
                    } else {
                        "Not Charging".cyan()
                    }
                );

                if device.is_wired() && !b.is_charging() {
                    println!(
                        "{}: mouse is plugged in, but battery is not charging",
                        "warning".yellow()
                    );
                }
            }
            Info::Sensor => {
                let s = Sensor::read(&device)?;

                let colored_preset = match s.mode() {
                    Mode::Basic => "basic".green(),
                    Mode::Competitive => "competitive".cyan(),
                    Mode::Max => "max".red(),
                };

                println!("Sensor is set to {} mode", colored_preset);
            }
        },
    }

    Ok(())
}
