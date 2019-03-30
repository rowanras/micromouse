reset

set term x11 1 noraise

unset key

stats 'spin.dat' using 1 name "X" nooutput

set xrange [X_max-10000:X_max]
set yrange [-10:10]

plot 'spin.dat' using 1:2 with lines, 'spin.dat' using 1:3 with lines

pause 0.01
reread

