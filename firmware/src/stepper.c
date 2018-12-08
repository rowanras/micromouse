#include <avr/io.h>
#include <avr/interrupt.h>
#include "stepper.h"

void init_left_stepper() {

    // set pin as output
    DDRD |= (1 << 6);
    DDRD |= (1 << 7);

    // set mode to phase correct pwm, updating pin
    // at top
    TCCR0A |= (1 << WGM00);
    TCCR0A &= ~(1 << WGM01);
    TCCR0B |= (1 << WGM02);

    // set compare output A to toggle pin
    TCCR0A |= (1 << COM0A0);
    TCCR0A &= ~(1 << COM0A1);

    TIMSK0 |= (1 << OCIE1A);

    //stop_left_stepper();
}

void set_left_direction(unsigned char direction) {
    if (direction) {
        PORTD |= (1 << 7);
    } else {
        PORTD &= ~(1 << 7);
    }
}

void stop_left_stepper() {

    // disable clock
    TCCR0B &= ~(1 << CS00);
    TCCR0B &= ~(1 << CS01);
    TCCR0B &= ~(1 << CS02);
}

volatile unsigned int left_steps = 0;

unsigned int get_left_steps() {
    return left_steps;
}

void set_left_stepper(unsigned char speed, unsigned int steps) {

    left_steps = steps;

    // set compare register to generate step pulses
    // at the right speed
    OCR0A = speed;

    // set clock to clk/256
    TCCR0B &= ~(1 << CS00);
    TCCR0B &= ~(1 << CS01);
    TCCR0B |= (1 << CS02);
}

ISR(TIMER0_COMPA_vect) {
    left_steps--;
    if (left_steps == 0) {
        stop_left_stepper();
    }
}

void init_right_stepper() {

    //set pin as output
    DDRB |= (1 << 3);
    DDRB |= (1 << 2);

    // set mode to phase correct pwm, updating pin
    // at top
    TCCR2A |= (1 << WGM20);
    TCCR2A &= ~(1 << WGM21);
    TCCR2B |= (1 << WGM22);

    // set compare output A to toggle pin
    TCCR2A |= (1 << COM2A0);
    TCCR2A &= ~(1 << COM2A1);

    TIMSK2 |= (1 << OCIE1A);

    //stop_right_stepper();
}

void set_right_direction(unsigned char direction) {
    if (direction) {
        PORTB |= (1 << 2);
    } else {
        PORTB &= ~(1 << 2);
    }
}

void stop_right_stepper() {

    // disable clock
    TCCR2B &= ~(1 << CS00);
    TCCR2B &= ~(1 << CS01);
    TCCR2B &= ~(1 << CS02);
}

volatile unsigned int right_steps = 0;

unsigned int get_right_steps() {
    return right_steps;
}

void set_right_stepper(unsigned char speed, unsigned int steps) {

    right_steps = steps;

    // set compare register to generate step pulses
    // at the right speed
    OCR2A = speed;

    // set clock to clk/256
    TCCR2B &= ~(1 << CS00);
    TCCR2B |= (1 << CS01);
    TCCR2B |= (1 << CS02);
}

ISR(TIMER2_COMPA_vect) {
    right_steps--;
    if (right_steps == 0) {
        stop_right_stepper();
    }
}
