reset

set term x11 1 noraise

unset key

stats 'pid.dat' using 1 name "X" nooutput
stats 'pid.dat' using 4 name "Y" nooutput

set xrange [X_max-5000:X_max]
#set yrange [Y_max-10000:Y_max]
set yrange [-1000:11000]
set y2range [-10000 : 10000]

set y2tics

plot 'pid.dat' using 1:2 with lines, 'pid.dat' using 1:3 with lines, 'pid.dat' using 1:4 with lines, 'pid.dat' using 1:5 with lines axes x1y2

pause 0.01
reread

