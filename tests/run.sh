#!/bin/bash

# Kill background jobs on exit.
trap 'kill $(jobs -p)' EXIT

# Run from script's directory.
cd "$(dirname "$0")"

# allow serial number to be overridden
if [ -z "$SERIAL" ]; then
  export SERIAL=04440001a0dec8b600000000000000000000000097969906 # ACM0 on plantmachine
fi

# Open GDB server with OpenOCD using the board's config.
openocd \
  -f interface/cmsis-dap.cfg \
  -f target/max78000.cfg \
  -c "adapter serial $SERIAL; init; reset init" & # ben's board

./uart_tester.py &

# Open tests in GDB.
cargo run --bin max78000-hal-tests -- -x .runner_gdb
