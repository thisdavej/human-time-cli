# human-time

A command-line tool for converting time durations to human-readable formats, built using the `human-time` crate.

## Installation

```bash
cargo install human-time-cli
```

## Usage

```bash
human-time [OPTIONS] <TIME_DURATION>
```

### Options

- `-u, --unit <UNIT>`: Specify the unit of the time duration (milli, micro). If not specified, the tool will first check the `default_time_value_units` in the config file. If no config file exists or it doesn't specify a unit, it defaults to seconds.
- `-c, --config`: Specify if there is a config file. (See [Configuration](#configuration) section for details on creating a config file.)

### Examples

- Convert 120 seconds to human-readable format:

    ```bash
    human-time 120
    ```

    Output:

    ```text
    2m
    ```

- Convert 500 milliseconds to human-readable format:

    ```bash
    human-time -u milli 1500
    ```

    Output:

    ```text
    1s,500ms
    ```

- Convert 7200 seconds using a config file:

    ```bash
    human-time 7200 -c
    ```

    (See [Configuration](#configuration) section for details on creating a config file.)

- Reading from stdin:

    ```bash
    echo 3600 | human-time
    ```

    Output:

    ```text
    1h
    ```

## Configuration

The `human-time` tool supports a configuration file named `human-time.toml`.  It can be placed in either of the following locations:

1. The same directory as the `human-time` executable.
2. Your home directory.

If a config file is found, the tool will use it to customize the output format and units.  If no config file is found, default values are used.

### `human-time.toml` Example

```toml
default_time_value_units = "seconds" # Default unit if -u is not provided

[formatting]
format = "{} {}"  # Format string for each time unit. Must contain exactly two "{}" placeholders. The first is for the time value, the second for the unit.
delimiter_text = ", " # Delimiter between time units when multiple units are displayed

[units]
d = "day(s)"
h = "hour(s)"
m = "minute(s)"
s = "second(s)"
ms = "millisecond(s)"
us = "microsecond(s)"
```

### Configuration Options

- `default_time_value_units`:  The default unit to use if the `-u` or `--unit` option is not provided on the command line.  Valid values are "seconds", "milliseconds", and "microseconds".
- `formatting.format`: The format string used to display each time unit.  It *must* contain exactly two `{}` placeholders. The first `{}` is replaced with the numeric time value, and the second `{}` is replaced with the unit string.
- `formatting.delimiter_text`: The string used to separate multiple time units when the duration is expressed in more than one unit (e.g., "1 hour, 30 minutes").
- `units`: A table of unit strings.  Singular/plural forms are automatically handled based on the time value if "(s)" is included (e.g., "second(s)").

## Input from Stdin

If no `TIME_DURATION` is provided as a command-line argument, `human-time` will attempt to read the duration from standard input (stdin). This allows you to pipe values to the command.

## Building from Source

If you want to build `human-time` from source, you'll need to have Rust and Cargo installed.

1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build`.

The executable will be located in the `target/debug` directory.
