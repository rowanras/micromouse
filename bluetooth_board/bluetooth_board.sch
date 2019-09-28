EESchema Schematic File Version 4
EELAYER 30 0
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
Wire Wire Line
	5600 2700 5650 2700
Wire Wire Line
	5600 2750 5600 2700
Wire Wire Line
	5500 2750 5600 2750
Wire Wire Line
	5500 2800 5500 2750
Wire Wire Line
	5400 2800 5500 2800
Wire Wire Line
	5550 2800 5650 2800
Wire Wire Line
	5550 2700 5550 2800
Wire Wire Line
	5400 2700 5550 2700
Wire Wire Line
	5500 2500 5650 2500
Wire Wire Line
	5500 2600 5500 2500
Wire Wire Line
	5400 2600 5500 2600
Wire Wire Line
	5550 2600 5650 2600
Wire Wire Line
	5550 2400 5550 2600
Wire Wire Line
	5400 2400 5550 2400
$Comp
L Connector:Conn_01x06_Male J2
U 1 1 5D8546FF
P 5850 2700
F 0 "J2" H 5822 2582 50  0000 R CNN
F 1 "Bluetooth" H 5822 2673 50  0000 R CNN
F 2 "Connector_PinSocket_2.54mm:PinSocket_1x06_P2.54mm_Horizontal" H 5850 2700 50  0001 C CNN
F 3 "~" H 5850 2700 50  0001 C CNN
	1    5850 2700
	-1   0    0    1   
$EndComp
Wire Wire Line
	5400 2500 5450 2500
Wire Wire Line
	5450 2500 5450 2450
Wire Wire Line
	5450 2450 5600 2450
Wire Wire Line
	5600 2450 5600 2400
Wire Wire Line
	5600 2400 5650 2400
$Comp
L Connector:Conn_01x06_Male J1
U 1 1 5D8548E6
P 5200 2600
F 0 "J1" H 5308 2981 50  0000 C CNN
F 1 "Serial" H 5308 2890 50  0000 C CNN
F 2 "Connector_PinSocket_2.54mm:PinSocket_1x06_P2.54mm_Horizontal" H 5200 2600 50  0001 C CNN
F 3 "~" H 5200 2600 50  0001 C CNN
	1    5200 2600
	1    0    0    -1  
$EndComp
Wire Wire Line
	5400 2900 5650 2900
$EndSCHEMATC
