EESchema Schematic File Version 4
EELAYER 29 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Connector:Conn_ARM_JTAG_SWD_10 J2
U 1 1 5D1812E8
P 5400 1900
F 0 "J2" H 5200 1900 50  0000 R CNN
F 1 "Conn_ARM_JTAG_SWD_10" V 4950 2300 50  0000 R CNN
F 2 "Connector_PinHeader_1.27mm:PinHeader_2x05_P1.27mm_Vertical_SMD" H 5450 1350 50  0001 L TNN
F 3 "http://infocenter.arm.com/help/topic/com.arm.doc.faqs/attached/13634/cortex_debug_connectors.pdf" V 5050 650 50  0001 C CNN
	1    5400 1900
	-1   0    0    -1  
$EndComp
$Comp
L Connector:Conn_ARM_JTAG_SWD_20 J1
U 1 1 5D181C0A
P 3850 1900
F 0 "J1" H 3321 1946 50  0000 R CNN
F 1 "Conn_ARM_JTAG_SWD_20" H 3321 1855 50  0000 R CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_2x10_P2.54mm_Vertical" H 4300 850 50  0001 L TNN
F 3 "http://infocenter.arm.com/help/topic/com.arm.doc.dui0499b/DUI0499B_system_design_reference.pdf" V 3500 650 50  0001 C CNN
	1    3850 1900
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0101
U 1 1 5D182F18
P 3750 2800
F 0 "#PWR0101" H 3750 2550 50  0001 C CNN
F 1 "GND" H 3755 2627 50  0000 C CNN
F 2 "" H 3750 2800 50  0001 C CNN
F 3 "" H 3750 2800 50  0001 C CNN
	1    3750 2800
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR0102
U 1 1 5D1832EE
P 3750 1000
F 0 "#PWR0102" H 3750 850 50  0001 C CNN
F 1 "+3V3" H 3765 1173 50  0000 C CNN
F 2 "" H 3750 1000 50  0001 C CNN
F 3 "" H 3750 1000 50  0001 C CNN
	1    3750 1000
	1    0    0    -1  
$EndComp
Wire Wire Line
	4900 1800 4450 1800
Wire Wire Line
	4450 1900 4900 1900
Wire Wire Line
	4450 2000 4900 2000
Wire Wire Line
	4900 2100 4450 2100
Wire Wire Line
	4900 1600 4700 1600
Wire Wire Line
	4700 1600 4700 1500
Wire Wire Line
	4700 1500 4450 1500
Wire Wire Line
	5500 2500 5500 2550
Wire Wire Line
	5500 2550 5400 2550
Wire Wire Line
	5400 2550 5400 2500
Wire Wire Line
	3750 1000 3750 1050
Wire Wire Line
	3750 1050 3850 1050
Wire Wire Line
	3850 1050 3850 1100
Connection ~ 3750 1050
Wire Wire Line
	3750 1050 3750 1100
Wire Wire Line
	3850 1050 5400 1050
Wire Wire Line
	5400 1050 5400 1300
Connection ~ 3850 1050
Wire Wire Line
	3750 2700 3750 2750
Wire Wire Line
	3750 2750 5500 2750
Wire Wire Line
	5500 2750 5500 2550
Connection ~ 3750 2750
Wire Wire Line
	3750 2750 3750 2800
Connection ~ 5500 2550
$Comp
L Device:LED D1
U 1 1 5D1864AC
P 6000 1550
F 0 "D1" V 6039 1433 50  0000 R CNN
F 1 "LED" V 5948 1433 50  0000 R CNN
F 2 "LED_SMD:LED_0603_1608Metric" H 6000 1550 50  0001 C CNN
F 3 "~" H 6000 1550 50  0001 C CNN
	1    6000 1550
	0    -1   -1   0   
$EndComp
$Comp
L Device:R R1
U 1 1 5D186DBD
P 6000 2200
F 0 "R1" H 6070 2246 50  0000 L CNN
F 1 "R" H 6070 2155 50  0000 L CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 5930 2200 50  0001 C CNN
F 3 "~" H 6000 2200 50  0001 C CNN
	1    6000 2200
	1    0    0    -1  
$EndComp
Wire Wire Line
	6000 1400 6000 1050
Wire Wire Line
	6000 1050 5400 1050
Connection ~ 5400 1050
Wire Wire Line
	6000 2350 6000 2750
Wire Wire Line
	6000 2750 5500 2750
Connection ~ 5500 2750
Wire Wire Line
	6000 1700 6000 2050
NoConn ~ 4450 1400
NoConn ~ 4450 1700
NoConn ~ 4450 2300
NoConn ~ 4450 2400
$EndSCHEMATC
