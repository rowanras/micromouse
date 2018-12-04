#include <avr/io.h>
#include <avr/interrupt.h>
#include "distance.h"

unsigned long capture_rise_time = 0;
unsigned long capture_fall_time = 0;
char capturing = 0;

void init_distance() {

    // Echo signal: 400us to 7000us

    // Set trigger pin to output
    DDRB |= (1<<1);

    TCCR1A = 0;
    TCCR1B = 0;

    TCCR1B &= ~(1<<CS10);
    TCCR1B |= (1<<CS11);
    TCCR1B &= ~(1<<CS12);

    OCR1AH = 0x00;
    OCR1AL = 0x01;

    TIMSK1 |= (1<<TOIE1);
    TIMSK1 |= (1<<OCIE1A);

    PORTB ^= (1<<1);

    TCCR1B |= (1<<ICNC1);
    TCCR1B |= (1<<ICNC1);
    TIMSK1 |= (1<<ICIE1);
}

ISR(TIMER1_OVF_vect) {
    PORTB |= (1<<1);
}

ISR(TIMER1_COMPA_vect) {
    PORTB &= ~(1<<1);
}

ISR(TIMER1_CAPT_vect) {
    if (capturing) {
        // End of distance pulse

        // save falling time
        capture_fall_time = ICR1L;
        capture_fall_time |= ICR1H << 8;

        // Set input capture edge select to rising edge
        TCCR1B |= (1<<ICES1);

        capturing = 0;
    } else {
        // Beginning of distance pulse

        // save rising time
        capture_rise_time = ICR1L;
        capture_rise_time |= ICR1H << 8;

        // Set input capture edge select to falling edge
        TCCR1B &= ~(1<<ICES1);

        capturing = 1;
    }
}

unsigned long get_distance() {
    return capture_fall_time - capture_rise_time;
}

