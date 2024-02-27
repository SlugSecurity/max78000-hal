#!/usr/bin/env python3

import serial
import os
import unittest
import time

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

	def tearDown(self):
		self.port.close()


if __name__ == '__main__':
	unittest.main()
