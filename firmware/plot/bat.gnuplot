reset

set term x11 1 noraise

unset key

stats 'data' using 1 name "X" nooutput

set xrange [X_max-500:X_max]
set yrange [0:10]

plot 'data' using 1:5 with lines

pause 0.01
reread

