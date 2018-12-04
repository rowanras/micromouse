#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "stepper.h"
#include "distance.h"
#include "uart.h"

int main() {

    DDRB |= (1 << 5);

    init_uart();
    uart_send_bytes("Initing\n");

    init_left_stepper();
    init_right_stepper();

    //set_left_stepper(10);
    //set_right_stepper(10);

    init_distance();

    sei();

    unsigned char d = 0;
    unsigned int n = 0;
    while(1) {
        _delay_ms(10);

        // blink led
        PORTB ^= (1 << 5);

        set_left_direction(d);
        set_right_direction(d);
        d = !d;

        unsigned int distance = get_distance();
        char number[16];
        itoa(distance, number, 10);
        uart_send_bytes(number);
        uart_send_byte('\n');
    }

}

