#!/bin/sh
rm -rf firmware/target
scp -rv firmware/* pi@192.168.0.30:/home/pi/Firmware