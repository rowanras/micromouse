reset

set term x11 1 noraise

unset key

stats 'motion.dat' using 1 name "X" nooutput

set xrange [X_max-5000:X_max]
set yrange [0:2000]
set y2range [-2000:2000]

plot 'motion.dat' using 1:2 with lines axes x1y2, 'motion.dat' using 1:3 with lines axes x1y1

pause 0.01
reread

