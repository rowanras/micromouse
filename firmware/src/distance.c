#include <avr/io.h>
#include <avr/interrupt.h>
#include "distance.h"

volatile unsigned long capture_rise_time = 0;
volatile unsigned long capture_fall_time = 0;
volatile char capturing = 0;
volatile unsigned char sensor_index = 0;

void init_distance() {

    // Echo signal: 400us to 7000us

    // Set trigger pin to output
    DDRB |= (1<<1);

    // Set select pins to output
    DDRD |= (1<<4);
    DDRD |= (1<<5);

    TCCR1A = 0;
    TCCR1B = 0;

    // 001 /1
    // 010 /8
    // 011 /64
    // 100 /256
    // 101 /1024
    TCCR1B &= ~(1<<CS10);
    TCCR1B |= (1<<CS11);
    TCCR1B &= ~(1<<CS12);

    OCR1AH = 0x00;
    OCR1AL = 0xff;

    OCR1BH = 0x01;
    OCR1BL = 0xff;

    TIMSK1 |= (1<<TOIE1);
    TIMSK1 |= (1<<OCIE1A);
    TIMSK1 |= (1<<OCIE1B);

    // Set echo pin to input
    PORTB ^= (1<<1);

    TCCR1B |= (1<<ICNC1);
    TCCR1B |= (1<<ICNC1);
    TIMSK1 |= (1<<ICIE1);
}

ISR(TIMER1_OVF_vect) {

    distances[sensor_index & 0x03] = capture_fall_time == 0 ? NO_SIGNAL : capture_fall_time - capture_rise_time;
    sensor_index += 1;

    unsigned char temp_port = PORTD;
    // clear needed bits
    temp_port &= ~(3 << 4);
    // set needed bits
    temp_port |= ((sensor_index & 0x03) << 4);
    // set back to actual port
    PORTD = temp_port;

    capture_rise_time = 0;
    capture_fall_time = 0;
}

ISR(TIMER1_COMPA_vect) {
    PORTB |= (1<<1);

    capture_rise_time = 0;
    capture_fall_time = 0;
}

ISR(TIMER1_COMPB_vect) {
    PORTB &= ~(1<<1);

    capture_rise_time = 0;
    capture_fall_time = 0;
}

ISR(TIMER1_CAPT_vect) {
    if (!capturing) {
        // Beginning of distance pulse

        // save rising time
        capture_rise_time = ICR1L;
        capture_rise_time |= ICR1H << 8;

        // Set input capture edge select to falling edge
        TCCR1B &= ~(1<<ICES1);

        capturing = 1;
    } else {
        // End of distance pulse

        // save falling time
        capture_fall_time = ICR1L;
        capture_fall_time |= ICR1H << 8;

        // Set input capture edge select to rising edge
        TCCR1B |= (1<<ICES1);

        capturing = 0;
    }
}
