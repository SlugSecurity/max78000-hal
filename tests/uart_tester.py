#!/usr/bin/env python3

import serial
import os
import unittest
import time

interjection = str.encode("""I'd just like to interject for a moment.  What you're referring to as Linux,
is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.
Linux is not an operating system unto itself, but rather another free component
of a fully functioning GNU system made useful by the GNU corelibs, shell
utilities and vital system components comprising a full OS as defined by POSIX.

Many computer users run a modified version of the GNU system every day,
without realizing it.  Through a peculiar turn of events, the version of GNU
which is widely used today is often called "Linux", and many of its users are
not aware that it is basically the GNU system, developed by the GNU Project.

There really is a Linux, and these people are using it, but it is just a
part of the system they use.  Linux is the kernel: the program in the system
that allocates the machine's resources to the other programs that you run.
The kernel is an essential part of an operating system, but useless by itself;
it can only function in the context of a complete operating system.  Linux is
normally used in combination with the GNU operating system: the whole system
is basically GNU with Linux added, or GNU/Linux.  All the so-called "Linux"
distributions are really distributions of GNU/Linux.
""")

print(len(interjection))

class UartTest(unittest.TestCase):
	def setUp(self):
		serial_id = os.environ['SERIAL']
		file = f'/dev/serial/by-id/usb-ARM_DAPLink_CMSIS-DAP_{serial_id}-if01'
		self.port = serial.Serial(file, 115200)
		time.sleep(0.25)

	def test_0_transmit(self):
		sent = self.port.read(14)
		self.assertEqual(sent, b'bleh bleh bleh')

	def test_1_receive(self):
		self.port.write(b'meow meow meow')
		self.port.flush()

	def test_2_recv_with_timeout(self):
		to_send = b'womp womp womp'
		for i in range(len(to_send)):
			time.sleep(0.1)
			self.port.write(to_send[i:i+1])
			self.port.flush()

	def test_3_recv_with_data_timeout(self):
		# tell board fifo has been flushed
		self.port.write(b'\xff')
		self.port.flush()

		to_send = b'womp womp womp'
		for i in range(len(to_send)):
			time.sleep(0.1)
			self.port.write(to_send[i:i+1])
			self.port.flush()

	def test_4_long_recv(self):
		total = 0
		to_send = interjection * 2
		while total < len(to_send):
			current = self.port.write(to_send[total:])
			total += current
		self.port.flush()

	def test_5_long_transmit(self):
		pass

	def tearDown(self):
		self.port.close()


if __name__ == '__main__':
	unittest.main()
