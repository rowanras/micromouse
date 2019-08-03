#!/bin/bash

openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg 2>1 > openocd.log &

arm-none-eabi-gdb -x openocd.gdb -q ../target/thumbv7em-none-eabihf/release/micromouse

