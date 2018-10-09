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

void uart_send_byte(unsigned char data) {
    while (!(UCSR0A & (1 << UDRE0)));
    UDR0 = data;
}

void uart_send_bytes(unsigned char *data) {
    unsigned char c;
    while (( c = *data++)) {
        uart_send_byte(c);
        _delay_ms(10);
    }
}
