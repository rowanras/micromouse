# micromouse2019
Micromouse for IEEE Micromouse competition. It uses an atmega328 as the processor, 3 ultrasonic distance sensors connected through a multiplexer, and two stepper motors and two stepper motor drivers.

The 3d printable body can be found in the `body/` folder, the circuit board design in the `board/` folder, and the firmware in the `firmware/` folder.

The micromouse uses the three ultrasonic sensors to detect walls around it. Because there are not enough inputs or timers in the atmega328 to handle all three ultrasonic sensors and the motors, the ultrasonic sensors go through a multiplexer. This allows all three to be read using just one interface to the atmega328. It also ensures that only one is pinging at a time, reducing interface. The echo responses are read with the input capture function of timer 1, and the trigger pulse is generated with the same timer. This leaves two timers to generate the step pulses for the stepper motors.

