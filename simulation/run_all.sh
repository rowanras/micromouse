#!/bin/bash

SIM=target/release/simulation

MAZE_DIR=micromouse_maze_tool/mazefiles/binary

MAZES=`ls $MAZE_DIR`

NAVS="CountingNavigate CountingDeadEndNavigate FloodFillNavigate FloodFillSquareNavigate FloodFillDeadEndNavigate FloodFillSquareDeadEndNavigate TwelvePartitionNavigate"

for maze in $MAZES; do for nav in $NAVS; do echo $MAZE_DIR/$maze $nav; done done |
    parallel --eta --progress --bar --results out --colsep ' ' $SIM {1} {2}

