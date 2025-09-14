# USB ECHO
Basic example based on an [Embassy example](https://github.com/embassy-rs/embassy/blob/embassy-rp-v0.8.0/examples/rp235x/src/bin/pio_uart.rs) for a Raspberry Pi Pico 2 W.

## Configuring the USB TTY on linux
The Raspberry Pi exposes two USB serial ports (e.g. /dev/ttyACM0 and /dev/ttyACM1). The second port is the echo port.
On the PC side this port must be configured as a raw port and the echo must be disabled:
`stty -F /dev/ttyACM1 raw -echo`

## Serial port permissions
By default, root is needed for serial port access. To give your user access, add yourself to the relevant group (i.e. `dialout` on Ubuntu, `uucp` on Arch Linux)