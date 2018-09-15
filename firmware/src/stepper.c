#include <avr/io.h>
#include "stepper.h"

void init_left_stepper() {

    // set pin as output
    DDRD |= (1 << 6);
    DDRD |= (1 << 5);

    // set mode to phase correct pwm, updating pin at top
    TCCR0A |= (1 << WGM00);
    TCCR0A &= ~(1 << WGM01);
    TCCR0B |= (1 << WGM02);

    // set compare output A to toggle pin
    TCCR0A |= (1 << COM0A0);
    TCCR0A &= ~(1 << COM0A1);

    //stop_left_stepper();
}

void set_left_direction(unsigned char direction) {
    if (direction) {
        PORTD |= (1 << 5);
    } else {
        PORTD &= ~(1 << 5);
    }
}

void stop_left_stepper() {

    // disable clock
    TCCR0B &= ~(1 << CS00);
    TCCR0B &= ~(1 << CS01);
    TCCR0B &= ~(1 << CS02);
}

void set_left_stepper(unsigned char speed) {

    // set clock to clk/256
    TCCR0B &= ~(1 << CS00);
    TCCR0B &= ~(1 << CS01);
    TCCR0B |= (1 << CS02);

    // set compare register to generate step pulses
    // at the right speed
    OCR0A = speed;
}

void init_right_stepper() {

    //set pin as output
    DDRB |= (1 << 3);
    DDRB |= (1 << 4);

    // set mode to phase correct pwm, updating pin at top
    TCCR2A |= (1 << WGM20);
    TCCR2A &= ~(1 << WGM21);
    TCCR2B |= (1 << WGM22);

    // set compare output A to toggle pin
    TCCR2A |= (1 << COM2A0);
    TCCR2A &= ~(1 << COM2A1);

    //stop_right_stepper();
}

void set_right_direction(unsigned char direction) {
    if (direction) {
        PORTB |= (1 << 4);
    } else {
        PORTB &= ~(1 << 4);
    }
}

void stop_right_stepper() {

    // disable clock
    TCCR2B &= ~(1 << CS00);
    TCCR2B &= ~(1 << CS01);
    TCCR2B &= ~(1 << CS02);
}

void set_right_stepper(unsigned char speed) {

    // set clock to clk/256
    TCCR2B &= ~(1 << CS00);
    TCCR2B |= (1 << CS01);
    TCCR2B |= (1 << CS02);

    // set compare register to generate step pulses
    // at the right speed
    OCR2A = speed;
}


