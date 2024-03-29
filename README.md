# Rust System Log ( Unix & Like-Unix )

This is a simple Rust program that demonstrates how to log a message to the system log on a Linux-based system using the syslog crate.

## Prerequisites

Before running this program, ensure you have the following installed on your system:

- Rust toolchain (including Cargo)
- Systemd or another syslog daemon (such as rsyslog or syslog-ng)

## Building the Program

1. Clone or download this repository to your local machine.
2. Navigate to the project directory in your terminal.

```bash
cd sysunixlog
```

3. Build the program using Cargo:

```bash
cargo build --release
```

## Running the Program

1. After successfully building the program, execute it using:

```bash
cargo run --release
```

or run the binary directly:

```bash
./target/release/sysunixlog
```

## Checking the System Log

### For systemd users:

Use the `journalctl` command to view logs managed by systemd. To filter logs from your program, you can use:

```bash
journalctl | grep myprogram
```

Replace `"myprogram"` with the name specified in your Rust code.

### For traditional syslog users:

Use the `tail` command to view the syslog file. For example:

```bash
sudo tail -f /var/log/syslog | grep myprogram
```

Replace `"myprogram"` with the name specified in your Rust code.

## Troubleshooting

- Ensure that the syslog service is running on your system.
- Check permissions if you encounter issues with logging to the system log.

If you encounter any issues, feel free to open an issue in this repository for assistance.
