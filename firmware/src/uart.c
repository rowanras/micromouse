#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "uart.h"

void init_uart() {
    UBRR0H = (BAUDRATE >> 8);
    UBRR0L = BAUDRATE;

    UCSR0B |= (1<<TXEN0);
    UCSR0B |= (1<<RXEN0);

    UCSR0C |= (1<<UCSZ00);
    UCSR0C |= (1<<UCSZ01);

    UCSR0B |= (1<<RXCIE0);
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

char rx_line_buf[RX_LINE_BUF_LEN][RX_CHAR_BUF_LEN];
unsigned char rx_char_buf_size = 0;
unsigned char rx_line_buf_size = 0;

unsigned char uart_lines_available() {
    return rx_line_buf_size;
}

unsigned char uart_read_line(char *buf) {

    for (unsigned char c = 0; c < RX_CHAR_BUF_LEN; c++) {
        buf[c] = rx_line_buf[0][c];
    }

    for (unsigned char l = 1; l < RX_LINE_BUF_LEN; l++) {
        for (unsigned char c = 0; c < RX_CHAR_BUF_LEN; c++) {
            rx_line_buf[l-1][c] = rx_line_buf[l][c];
        }
    }

    rx_line_buf_size -= 1;

    return rx_line_buf_size;
}

ISR(USART_RX_vect) {
    char in = UDR0;
    if (
        rx_char_buf_size < RX_CHAR_BUF_LEN &&
        rx_line_buf_size < RX_LINE_BUF_LEN
    ) {
        rx_line_buf[rx_line_buf_size][rx_char_buf_size] = in;
        rx_char_buf_size += 1;
    }

    if (in == '\n' && rx_line_buf_size < RX_LINE_BUF_LEN-1) {
        rx_line_buf[rx_line_buf_size][rx_char_buf_size] = '\0';
        rx_line_buf_size += 1;
        rx_char_buf_size = 0;
    }
}
