reset

set term x11 1 noraise

unset key

stats 'data' using 1 name "X" nooutput
stats 'data' using 4 name "Y" nooutput

set xrange [X_max-500:X_max]
#set yrange [Y_max-10000:Y_max]
set yrange [-1000:11000]
set y2range [-1000 : 1000]

plot 'data' using 1:2 with lines, 'data' using 1:3 with lines, 'data' using 1:4 with lines

pause 0.01
reread

