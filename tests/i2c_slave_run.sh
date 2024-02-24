#!/bin/bash

# Kill background jobs on exit.
trap 'kill $(jobs -p)' EXIT

# Run from script's directory.
cd "$(dirname "$0")"

# Open GDB server with OpenOCD using the board's config.
/opt/MaximSDK/Tools/OpenOCD/openocd -s /opt/MaximSDK/Tools/OpenOCD/scripts \
  -f interface/cmsis-dap.cfg \
  -f target/max78000.cfg \
  -c "tcl_port 40777; telnet_port 40778; gdb_port 40779; adapter serial 04440001a0dec8b600000000000000000000000097969906; init; reset init" &

# Open tests in GDB.
cargo run --bin i2c_slave_test -- -x .runner_gdb_slave
