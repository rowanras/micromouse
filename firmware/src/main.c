#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "stepper.h"
#include "distance.h"
#include "uart.h"
//#include "motion.h"

#define MAX_ARGS 8
#define MAX_ARG_LEN 8

char **split(char *buf) {
    static char *args[MAX_ARGS] = {""};

    args[0] = buf;

    unsigned char c = 0;
    unsigned char a = 1;

    while (buf[c] != '\0') {

        if (*(buf+c) == ' ') {
            *(buf+c) = '\0';

            args[a] = buf+c+1;

            a++;
        }

        c++;
    }

    return args;
}

unsigned char string_comp(char *str1, char *str2) {
    unsigned char c = 0;

    while (str1[c] != '\0' && str2[c] != '\0') {
        if (str1[c] != str2[c]) return 0;
        c++;
    }

    return 1;
}

unsigned char string_len(char * str) {
    unsigned char c = 0;
    while (str[c] != '\0') c++;
    return c;
}

unsigned int string_to_dec(char *str) {

    unsigned char c = 0;
    unsigned int dec = 0;

    while (str[c] != '\0' ) {
        dec *= 10;
        dec += (str[c]-'0');
        c++;
    }

    return dec;
}

int main() {

    DDRB |= (1 << 5);
    DDRB |= (1 << 4);

    init_uart();
    uart_send_bytes("Initing\n");

    init_left_stepper();
    init_right_stepper();

    init_distance();

    sei();

    while(1) {

        if (uart_lines_available() > 0) {
            PORTB ^= (1 << 4);
            char line_buf[RX_CHAR_BUF_LEN+1] = "12345678";

            uart_read_line(line_buf);

            line_buf[8] = '\0';

            uart_send_bytes("echo: ");
            uart_send_bytes(line_buf);
            uart_send_byte('\n');

            char **args = split(line_buf);

            /*
            for (int a = 0; a < MAX_ARGS; a++) {
                uart_send_byte(a+'0');
                uart_send_bytes(": ");
                uart_send_bytes(args[a]);
                uart_send_byte('\n');
            }
            */

            if (string_comp(args[0], "f")) {
                unsigned int speed = string_to_dec(args[1]);
                unsigned int distance = string_to_dec(args[2]);

                uart_send_bytes("forward\n");
                uart_send_unsigned_long(speed);
                uart_send_byte('\n');
                uart_send_unsigned_long(distance);
                uart_send_byte('\n');

                forward(speed, distance);
            } else if (string_comp(args[0], "b")) {
                unsigned int speed = string_to_dec(args[1]);
                unsigned int distance = string_to_dec(args[2]);

                uart_send_bytes("backward\n");
                uart_send_unsigned_long(speed);
                uart_send_byte('\n');
                uart_send_unsigned_long(distance);
                uart_send_byte('\n');

                backward(speed, distance);
            } else if (string_comp(args[0], "l")) {
                unsigned int speed = string_to_dec(args[1]);
                unsigned int distance = string_to_dec(args[2]);

                uart_send_bytes("left\n");
                uart_send_unsigned_long(speed);
                uart_send_byte('\n');
                uart_send_unsigned_long(distance);
                uart_send_byte('\n');

                left(speed, distance);
            } else if (string_comp(args[0], "r")) {
                unsigned int speed = string_to_dec(args[1]);
                unsigned int distance = string_to_dec(args[2]);

                uart_send_bytes("right\n");
                uart_send_unsigned_long(speed);
                uart_send_byte('\n');
                uart_send_unsigned_long(distance);
                uart_send_byte('\n');

                right(speed, distance);
            } else {
                uart_send_bytes("not\n");
            }
        }

        //_delay_ms(500);

        // blink led
        PORTB ^= (1 << 5);

        /*
        for (int i = 0; i < 3; i++) {
            unsigned int distance = distances[i];
            uart_send_unsigned_long(distance);
            uart_send_byte(' ');
            uart_send_byte(distance < 2000 ? '|' : ' ');
            uart_send_byte(' ');
        }

        uart_send_byte('\n');
        */
    }
}

