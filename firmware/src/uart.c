#include <avr/io.h>
#include <util/delay.h>
#include "uart.h"

void init_uart() {
    UBRR0H = (BAUDRATE >> 8);
    UBRR0L = BAUDRATE;

    UCSR0B |= (1<<TXEN0);
    UCSR0B |= (1<<RXEN0);

    UCSR0C |= (1<<UCSZ00);
    UCSR0C |= (1<<UCSZ01);
}

void uart_send_byte(char data) {
    while (!(UCSR0A & (1 << UDRE0)));
    UDR0 = data;
}

void uart_send_bytes(char *data) {
    char c;
    while (( c = *data++)) {
        uart_send_byte(c);
        _delay_ms(10);
    }
}

void uart_send_unsigned_long(unsigned long number) {
    char buffer[7] = "000000";

    unsigned long to_convert = number;

    int i = 5;
    while (to_convert != 0 && i >=0) {
        buffer[i] = to_convert % 10 + '0';
        to_convert /= 10;
        i--;
    }

    uart_send_bytes(buffer);
}
