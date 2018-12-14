#include "stepper.h"
#include "motion.h"

#define STEPS_PER_MM 24
#define STEPS_PER_DEGREE 23

void forward(unsigned char speed, unsigned int distance) {
    set_left_direction(1);
    set_right_direction(1);

    set_left_stepper(speed, distance * STEPS_PER_MM);
    set_right_stepper(speed, distance * STEPS_PER_MM);
}

void backward(unsigned char speed, unsigned int distance) {
    set_left_direction(0);
    set_right_direction(0);

    set_left_stepper(speed, distance * STEPS_PER_MM);
    set_right_stepper(speed, distance * STEPS_PER_MM);
}

void left(unsigned char speed, unsigned int distance) {
    set_left_direction(1);
    set_right_direction(0);

    set_left_stepper(speed, distance * STEPS_PER_DEGREE);
    set_right_stepper(speed, distance * STEPS_PER_DEGREE);
}

void right(unsigned char speed, unsigned int distance) {
    set_left_direction(0);
    set_right_direction(1);

    set_left_stepper(speed, distance * STEPS_PER_DEGREE);
    set_right_stepper(speed, distance * STEPS_PER_DEGREE);
}

