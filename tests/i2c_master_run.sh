#!/bin/bash

# Kill background jobs on exit.
trap 'kill $(jobs -p)' EXIT

# Run from script's directory.
cd "$(dirname "$0")"

# Open GDB server with OpenOCD using the board's config.
/opt/MaximSDK/Tools/OpenOCD/openocd -s /opt/MaximSDK/Tools/OpenOCD/scripts \
  -f interface/cmsis-dap.cfg \
  -f target/max78000.cfg \
  -c "adapter serial 0444000158dec8cf00000000000000000000000097969906; init; reset init" &

# Open tests in GDB.
cargo run --bin i2c_master_test -- -x .runner_gdb
