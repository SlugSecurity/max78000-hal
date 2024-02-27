#!/usr/bin/env python3

import serial
from sys import argv

serial_id = argv[1]
port = f'/dev/serial/by-id/usb-ARM_DAPLink_CMSIS-DAP_{serial_id}-if01'
