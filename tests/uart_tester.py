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
""") * 2
for i in range(256):
	interjection += bytes([i])

ego = str.encode("""don't interpret it as ego. Long is computer researcher more than a professor, he's the highest profile person to teach us the way of the industry, he would be better off doing research for the betterment of the computer science field but he decide to nurture us students to have a good impact on the industry as a whole. If you are wondering why that's a good thing, I'll give you 2 sides of the picture to compare: 
1. Veenstra has 240 students, that means complete chaos with grading, less interactions with students, TAs are gonna be overwhelmed, etc, etc, pretty much what's happening in this class. Unless he can manage it well (which is a pretty tall task) 
2. Long has 51 students, that means 51 students are gonna get the content closest to the industry to get jobs from a high profile person of the campus. While being chill with the grading, more engagement with the students, etc. 

I don't usually make comments like this, but I had to get it out there cause he's being misunderstood hard by alot of people
""") * 2
for i in range(256):
	ego += bytes([i])

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
		# 2 oscillators * 13 prescalers
		for i in range(2 * 13):
			time.sleep(0.1)
			self.port.write(interjection)
			self.port.flush()

	def test_5_long_transmit(self):
		for i in range(2 * 13):
			time.sleep(0.1)
			sent = self.port.read(len(ego))
			self.assertEqual(sent, ego)

	def test_6_recv_line(self):
		self.port.write(b'short line\n'
				  + b'another short line\r'
				  + b'CRLF line\r\n'
				  + b'a line that fills up the buffer before a newline\n')
		self.port.flush()

	def test_5_recv_formatted1(self):
		self.port.reset_input_buffer()
		sent_test_string = b'%debug: this is a debug message%'
		sent = self.port.read(len(sent_test_string))
		self.assertEqual(sent, sent_test_string)

	def test_6_recv_formatted3(self):
		self.port.reset_input_buffer()
		sent_test_string = b'This is a uart test, what is uart? uart is a your art. uart is universal art. uart is uncanny art.'
		sent = self.port.read(len(sent_test_string))
		self.assertEqual(sent, sent_test_string)

	def test_7_recv_formatted2(self):
		self.port.reset_input_buffer()
		sent_test_string = b'%info: DATE>1/1/1970%'
		sent = self.port.read(len(sent_test_string))
		self.assertEqual(sent, sent_test_string)

	def tearDown(self):
		self.port.close()


if __name__ == '__main__':
	unittest.main()
