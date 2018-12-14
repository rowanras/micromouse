
#ifndef UART_H
#define UART_H

#ifndef F_CPU
#define F_CPU 16000000
#endif

#define BAUD 9600
#define BAUDRATE ((F_CPU)/(16UL*BAUD)-1)

#define RX_CHAR_BUF_LEN 8
#define RX_LINE_BUF_LEN 8

void init_uart();
void uart_send_byte(char data);
void uart_send_bytes(char *data);
void uart_send_unsigned_long(unsigned long number);

unsigned char uart_lines_available();
unsigned char uart_read_line(char *buf);

#endif

