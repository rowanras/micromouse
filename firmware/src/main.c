#include <avr/io.h>
#include <util/delay.h>
#include "stepper.h"

int main() {

    DDRB |= (1 << 5);

    init_left_stepper();
    init_right_stepper();
    set_left_stepper(10);
    set_right_stepper(10);

    unsigned char d = 0;
    while(1) {
        _delay_ms(1000);
        PORTB ^= (1 << 5);
        set_left_direction(d);
        d = !d;
    }

}

