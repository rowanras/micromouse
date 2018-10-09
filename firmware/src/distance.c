#include <avr/io.h>
#include "distance.h"

unsigned int capture_rise_time = 0;
unsigned int capture_fall_time = 0;
char capturing = 0;

void init_distance() {

    // Echo signal: 400us to 7000us

    // 15.2597Hz pulse frequency
    // 130us per tick

    // Set trigger pin to output
    DDRB |= (1<<1);

    // Set capture pin as input
    DDRB &= ~(1<<0);

    // Set compare output mode to clear on compare match and set on BOTTOM
    TCCR1A &= ~(1<<COM1A0);
    TCCR1A |= (1<<COM1A1);

    // Set waveform generation mode to fast pwm, top at 0x03FF
    TCCR1A |= (1<<WGM10);
    TCCR1A |= (1<<WGM11);
    TCCR1B |= (1<<WGM12);
    TCCR1B &= ~(1<<WGM13);

    // Set clock source to clk/64
    TCCR1B |= (1<<CS10);
    TCCR1B &= ~(1<<CS11);
    TCCR1B |= (1<<CS12);

    // Set output compare to 25
    // Results in 101.732us pulse
    OCR1AL = 1;
    OCR1AH = 0;

    // Set input capture edge select to rising edge
    TCCR1B |= (1<<ICES1);

    // enable input capture noise canceling
    //TCCR1B |= (1<<ICNC1);

    // enable input capture interrupt
    TIMSK1 |= (1<<ICIE1);
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

unsigned int get_distance() {
    return capture_fall_time - capture_rise_time;
}

