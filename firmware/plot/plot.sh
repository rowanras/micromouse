#!/bin/bash


tail -f $1 | cat -n > data &
gnuplot plot.gnuplot

trap 'kill $(jobs -p)' EXIT

