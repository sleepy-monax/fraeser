#!/bin/sh
rm -rf firmware/target
scp -rv firmware/* pi@192.168.43.226:/home/pi/Firmware