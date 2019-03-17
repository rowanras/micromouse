#!/bin/bash


tail -f $1 | awk '
$1 ~ "start" { print > "data"; close("data") }
$1 !~ "start" { print >> "data"; close("data") }' &

gnuplot $2.gnuplot

trap 'kill $(jobs -p)' EXIT

