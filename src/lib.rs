use argh::FromArgs;
use human_time::ToHumanTimeString;
use regex_lite::Regex;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Convert time duration to human-readable format
#[derive(FromArgs)]
#[argh(
    name = "human-time",
    description = "Converts a time duration to a human-readable format"
)]
pub struct Args {
    /// the time duration to convert
    #[argh(positional)]
    pub time_value: Option<u64>,

    /// specify the unit of the time duration (milli, micro). If not specified, defaults to seconds.
    #[argh(option, short = 'u', long = "unit")]
    pub unit: Option<String>,

    /// specify if there is a config file (the name will be human-time.toml)
    #[argh(switch, short = 'c', long = "config")]
    pub config: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub default_time_value_units: String,
    pub formatting: Formatting,
    pub units: Units,
}

#[derive(Deserialize)]
pub struct Formatting {
    pub format: String,
    pub delimiter_text: String,
}

#[derive(Deserialize)]
pub struct Units {
    pub d: String,
    pub h: String,
    pub m: String,
    pub s: String,
    pub ms: String,
    pub us: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_time_value_units: "seconds".to_string(),
            formatting: Formatting {
                format: "{}{}".to_string(),
                delimiter_text: ",".to_string(),
            },
            units: Units {
                d: "d".to_string(),
                h: "h".to_string(),
                m: "m".to_string(),
                s: "s".to_string(),
                ms: "ms".to_string(),
                us: "µs".to_string(),
            },
        }
    }
}

const MILLI_REGEX: &str = r"^(?:milli(?:second|sec)?s?|ms)$";
const MICRO_REGEX: &str = r"^micro(?:second|sec)?s?$";
const SEC_REGEX: &str = r"^(?:sec(?:ond)?s?|s)$";

pub fn print_error_and_exit(error_message: &str) -> ! {
    eprintln!("{error_message}");
    eprintln!(
        r#"Usage: human-time-cli [OPTIONS] <TIME_DURATION>
Options:
  -u, --unit <UNIT>       specify the unit of the time value (milli, micro). If not specified, defaults to seconds.
  -c, --config            specify if there is a config file"#
    );
    std::process::exit(1);
}

pub fn validate_config(config: &Config) -> Result<(), String> {
    let unit = config.default_time_value_units.to_lowercase();
    let milli_regex = Regex::new(MILLI_REGEX).unwrap();
    let micro_regex = Regex::new(MICRO_REGEX).unwrap();
    let sec_regex = Regex::new(SEC_REGEX).unwrap();

    if !(milli_regex.is_match(&unit) || micro_regex.is_match(&unit) || sec_regex.is_match(&unit)) {
        return Err(format!(
            "Invalid default_time_value_units: {}. Valid options are: milliseconds, microseconds, or seconds.",
            config.default_time_value_units
        ));
    }

    // Check if formatting.format contains exactly two sets of {}
    let format = &config.formatting.format;
    let placeholder_count = format.matches("{}").count();
    if placeholder_count != 2 {
        return Err(format!(
            "Invalid formatting.format: {}. It must contain exactly two sets of {{}}.",
            format
        ));
    }

    Ok(())
}

pub fn convert_time(time_value: u64, unit: Option<&str>) -> Result<Duration, String> {
    let unit = unit.unwrap_or("sec").to_lowercase();

    let milli_regex = Regex::new(MILLI_REGEX).unwrap();
    let micro_regex = Regex::new(MICRO_REGEX).unwrap();
    let sec_regex = Regex::new(SEC_REGEX).unwrap();

    let duration = if milli_regex.is_match(&unit) {
        Duration::from_millis(time_value)
    } else if micro_regex.is_match(&unit) {
        Duration::from_micros(time_value)
    } else if sec_regex.is_match(&unit) {
        Duration::from_secs(time_value)
    } else {
        return Err(format!(
            "Invalid unit '{}'. Please specify one of: milli, micro, or leave empty for seconds.",
            unit
        ));
    };

    Ok(duration)
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Config {
    match fs::read_to_string(&path) {
        Ok(config_content) => match toml::from_str(&config_content) {
            Ok(config) => {
                if let Err(err) = validate_config(&config) {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
                config
            }
            Err(_) => {
                eprintln!("Error: Failed to parse the config file.");
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!(
                "Error: Config file not found at this location: {}",
                path.as_ref().display()
            );
            std::process::exit(1);
        }
    }
}

pub fn find_config_file() -> Option<PathBuf> {
    let exe_path = env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;
    let config_file_name = "human-time.toml";

    let exe_config_path = exe_dir.join(config_file_name);
    if exe_config_path.exists() {
        return Some(exe_config_path);
    }

    let home_dir = dirs::home_dir()?;
    let home_config_path = home_dir.join(config_file_name);
    if home_config_path.exists() {
        return Some(home_config_path);
    }
    None
}

pub fn format_duration(time_value: u64, unit: &str, config: &Config) -> Result<String, String> {
    match convert_time(time_value, Some(unit)) {
        Ok(duration) => {
            let formatted_duration = duration.to_human_time_string_with_format(
                |n, unit| {
                    let unit_str = match unit {
                        "d" => &config.units.d,
                        "h" => &config.units.h,
                        "m" => &config.units.m,
                        "s" => &config.units.s,
                        "ms" => &config.units.ms,
                        _ => &config.units.us,
                    };

                    let unit_str = if n == 1 {
                        unit_str.replace("(s)", "")
                    } else {
                        unit_str.replace("(s)", "s")
                    };

                    config
                        .formatting
                        .format
                        .replacen("{}", &n.to_string(), 1)
                        .replacen("{}", &unit_str, 1)
                },
                |acc, item| format!("{}{}{}", acc, config.formatting.delimiter_text, item),
            );
            Ok(formatted_duration)
        }
        Err(err) => Err(err),
    }
}
