#!/bin/bash


tail -f $1 | cat -n > data &
gnuplot $2.gnuplot

trap 'kill $(jobs -p)' EXIT

