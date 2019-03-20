reset

set term x11 1 noraise

unset key

stats 'motion.dat' using 1 name "X" nooutput

set xrange [X_max-5000:X_max]
set yrange [-5000:5000]
set y2range [-5:5]

plot 'motion.dat' using 1:6 with lines axes x1y2, 'motion.dat' using 1:7 with lines axes x1y1

pause 0.01
reread

