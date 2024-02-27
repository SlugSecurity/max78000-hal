#!/bin/bash

id=$1
port=/dev/serial/by-id/usb-ARM_DAPLink_CMSIS-DAP_$id-if01
stty_settings="-F $port ignbrk -brkint -icrnl -imaxbel -opost -onlcr -isig -icanon -iexten -echo -echoe -echok -echoctl -echoke 115200"

cat <<EOF
=============
uart_tester
$id
$port
=============
EOF

dd_from() {
	stty $stty_settings
	dd if=$port bs=1 status=none $@
}

dd_to() {
	stty $stty_settings
	dd of=$port bs=1 status=none
}

timeout_command() {
	timeout 1 $@
	exit=$?
	if [ $exit == 124 ]; then
		echo 'fail (timed out)'
		exit 1
	fi
}

received=$(dd_from count=14)
if [ "$received" != 'bleh bleh bleh' ]; then
	echo "host side UART tests failed: expected 'bleh bleh bleh', got '$received'"
	exit 1
fi

printf 'meow meow meow' | dd_to count=14
