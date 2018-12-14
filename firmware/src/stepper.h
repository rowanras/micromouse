
#ifndef STEPPER_H
#define STEPPER_H

void init_left_stepper();
void set_left_direction(unsigned char direction);
void stop_left_stepper();
void set_left_stepper(unsigned char speed, unsigned int steps);

void init_right_stepper();
void set_right_direction(unsigned char direction);
void stop_right_stepper();
void set_right_stepper(unsigned char speed, unsigned int steps);

#endif
