#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "stepper.h"
#include "distance.h"
#include "uart.h"
//#include "motion.h"

int main() {

    DDRB |= (1 << 5);

    init_uart();
    uart_send_bytes("Initing\n");

    init_left_stepper();
    init_right_stepper();

    //set_left_stepper(10);
    //set_right_stepper(10);
    set_left_stepper(10, 20000);
    set_right_stepper(10, 10000);

    init_distance();

    sei();

    while(1) {

        //_delay_ms(500);

        // blink led
        PORTB ^= (1 << 5);

        for (int i = 0; i < 3; i++) {
            unsigned int distance = distances[i];
            uart_send_unsigned_long(distance);
            uart_send_byte(' ');
            uart_send_byte(distance < 2000 ? '|' : ' ');
            uart_send_byte(' ');
        }

        uart_send_byte('\n');
    }
}

