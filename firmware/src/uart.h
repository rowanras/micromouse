#ifndef UART_H
#define UART_H

#ifndef F_CPU
#define F_CPU 16000000
#endif

#define BAUD 9600
#define BAUDRATE ((F_CPU)/(16UL*BAUD)-1)

void init_uart();
void uart_send_byte(unsigned char data);
void uart_send_bytes(unsigned char *data);

#endif

