
#ifndef STEPPER_H
#define STEPPER_H

#define STEPPER_MAX_STEPS 

void init_left_stepper();
void set_left_direction(unsigned char direction);
void stop_left_stepper();
unsigned int get_left_steps();
void set_left_stepper(unsigned char speed, unsigned int steps);

void init_right_stepper();
void set_right_direction(unsigned char direction);
void stop_right_stepper();
unsigned int get_right_steps();
void set_right_stepper(unsigned char speed, unsigned int steps);

#endif
