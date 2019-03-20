#!/bin/bash

rm $2.dat

tail -f $1 | awk -v datafile=$2.dat '
$1 ~ "start" { print > datafile; close(datafile) }
$1 !~ "start" { print >> datafile; close(datafile) }' &

gnuplot $2.gnuplot

trap 'kill $(jobs -p)' EXIT

