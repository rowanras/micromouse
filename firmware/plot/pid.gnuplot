reset

set term x11 1 noraise

unset key

stats 'pid.dat' using 1 name "X" nooutput

set xrange [X_max-5000:X_max]
set yrange [-5:5]

plot 'pid.dat' using 1:2 with lines, 'pid.dat' using 1:3 with lines, 'pid.dat' using 1:4 with lines, 'pid.dat' using 1:5 with lines

pause 0.01
reread

