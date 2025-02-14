use human_time_cli::*;
use std::time::Duration;

#[test]
fn test_convert_time_seconds() {
    let result = convert_time(3600, Some("sec")).unwrap();
    assert_eq!(result, Duration::from_secs(3600));
}

#[test]
fn test_convert_time_milliseconds() {
    let result = convert_time(3600, Some("milli")).unwrap();
    assert_eq!(result, Duration::from_millis(3600));
}

#[test]
fn test_convert_time_microseconds() {
    let result = convert_time(3600, Some("micro")).unwrap();
    assert_eq!(result, Duration::from_micros(3600));
}

#[test]
fn test_convert_time_invalid_unit() {
    let result = convert_time(3600, Some("invalid"));
    assert!(result.is_err());
}

#[test]
fn test_validate_config_valid() {
    let config = Config {
        default_time_value_units: "seconds".to_string(),
        formatting: Formatting {
            format: "{} {}".to_string(),
            delimiter_text: ", ".to_string(),
        },
        units: Units {
            d: "day(s)".to_string(),
            h: "hour(s)".to_string(),
            m: "minute(s)".to_string(),
            s: "second(s)".to_string(),
            ms: "millisecond(s)".to_string(),
            us: "microsecond(s)".to_string(),
        },
    };
    let result = validate_config(&config);
    assert!(result.is_ok());
}

#[test]
fn test_validate_config_invalid() {
    let config = Config {
        default_time_value_units: "invalid".to_string(),
        formatting: Formatting {
            format: "{} {}".to_string(),
            delimiter_text: ", ".to_string(),
        },
        units: Units {
            d: "day(s)".to_string(),
            h: "hour(s)".to_string(),
            m: "minute(s)".to_string(),
            s: "second(s)".to_string(),
            ms: "millisecond(s)".to_string(),
            us: "microsecond(s)".to_string(),
        },
    };
    let result = validate_config(&config);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid default_time_value_units: invalid. Valid options are: milliseconds, microseconds, or seconds."
    );
}

#[test]
fn test_validate_config_format_invalid() {
    let config = Config {
        default_time_value_units: "seconds".to_string(),
        formatting: Formatting {
            format: "{}".to_string(), // missing second {} in the format string
            delimiter_text: ", ".to_string(),
        },
        units: Units {
            d: "day(s)".to_string(),
            h: "hour(s)".to_string(),
            m: "minute(s)".to_string(),
            s: "second(s)".to_string(),
            ms: "millisecond(s)".to_string(),
            us: "microsecond(s)".to_string(),
        },
    };
    let result = validate_config(&config);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid formatting.format: {}. It must contain exactly two sets of {}."
    );
}

#[test]
fn test_read_config_valid() {
    let config_content = r#"
        default_time_value_units = "seconds"
        [formatting]
        format = "{} {}"
        delimiter_text = ", "
        [units]
        d = "day(s)"
        h = "hour(s)"
        m = "minute(s)"
        s = "second(s)"
        ms = "millisecond(s)"
        us = "microsecond(s)"
    "#;
    let config: Config = toml::from_str(config_content).unwrap();
    let result = validate_config(&config);
    assert!(result.is_ok());
}

#[test]
fn test_read_config_invalid() {
    let config_content = r#"
        default_time_value_units = "invalid"
        [formatting]
        format = "{} {}"
        delimiter_text = ", "
        [units]
        d = "day(s)"
        h = "hour(s)"
        m = "minute(s)"
        s = "second(s)"
        ms = "millisecond(s)"
        us = "microsecond(s)"
    "#;
    let config: Config = toml::from_str(config_content).unwrap();
    let result = validate_config(&config);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid default_time_value_units: invalid. Valid options are: milliseconds, microseconds, or seconds."
    );
}

#[test]
fn test_format_duration_seconds() {
    let config = Config::default();
    let result = format_duration(3600, "sec", &config).unwrap();
    assert_eq!(result, "1 hour");
}

#[test]
fn test_format_duration_seconds2() {
    let config = Config::default();
    let result = format_duration(7200, "sec", &config).unwrap();
    assert_eq!(result, "2 hours");
}

#[test]
fn test_format_duration_seconds3() {
    let mut config = Config::default();
    config.units.h = "hour".to_string();
    let result = format_duration(7200, "sec", &config).unwrap();
    assert_eq!(result, "2 hour");
}

#[test]
fn test_format_duration_milliseconds() {
    let config = Config::default();
    let result = format_duration(3600, "milli", &config).unwrap();
    assert_eq!(result, "3 seconds, 600 milliseconds");
}

#[test]
fn test_format_duration_microseconds() {
    let config = Config::default();
    let result = format_duration(3600, "micro", &config).unwrap();
    assert_eq!(result, "3 milliseconds, 600 microseconds");
}

#[test]
fn test_format_duration_invalid_unit() {
    let config = Config::default();
    let result = format_duration(3600, "invalid", &config);
    assert!(result.is_err());
}
