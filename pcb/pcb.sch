EESchema Schematic File Version 4
LIBS:pcb-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 45
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
L power:PWR_FLAG #FLG01
U 1 1 5C20AF11
P 1100 1300
F 0 "#FLG01" H 1100 1375 50  0001 C CNN
F 1 "PWR_FLAG" H 1100 1474 50  0000 C CNN
F 2 "" H 1100 1300 50  0001 C CNN
F 3 "~" H 1100 1300 50  0001 C CNN
	1    1100 1300
	1    0    0    -1  
$EndComp
$Comp
L power:PWR_FLAG #FLG04
U 1 1 5C20AF23
P 2200 1300
F 0 "#FLG04" H 2200 1375 50  0001 C CNN
F 1 "PWR_FLAG" H 2200 1474 50  0000 C CNN
F 2 "" H 2200 1300 50  0001 C CNN
F 3 "~" H 2200 1300 50  0001 C CNN
	1    2200 1300
	1    0    0    -1  
$EndComp
$Comp
L power:PWR_FLAG #FLG02
U 1 1 5C20AF2E
P 1100 1850
F 0 "#FLG02" H 1100 1925 50  0001 C CNN
F 1 "PWR_FLAG" H 1100 2024 50  0000 C CNN
F 2 "" H 1100 1850 50  0001 C CNN
F 3 "~" H 1100 1850 50  0001 C CNN
	1    1100 1850
	1    0    0    -1  
$EndComp
$Comp
L power:PWR_FLAG #FLG05
U 1 1 5C20AF39
P 2200 1850
F 0 "#FLG05" H 2200 1925 50  0001 C CNN
F 1 "PWR_FLAG" H 2200 2024 50  0000 C CNN
F 2 "" H 2200 1850 50  0001 C CNN
F 3 "~" H 2200 1850 50  0001 C CNN
	1    2200 1850
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR02
U 1 1 5C20AF85
P 1100 1900
F 0 "#PWR02" H 1100 1650 50  0001 C CNN
F 1 "GND" H 1105 1727 50  0000 C CNN
F 2 "" H 1100 1900 50  0001 C CNN
F 3 "" H 1100 1900 50  0001 C CNN
	1    1100 1900
	1    0    0    -1  
$EndComp
$Comp
L power:+9V #PWR01
U 1 1 5C20B079
P 1100 1350
F 0 "#PWR01" H 1100 1200 50  0001 C CNN
F 1 "+9V" H 1115 1523 50  0000 C CNN
F 2 "" H 1100 1350 50  0001 C CNN
F 3 "" H 1100 1350 50  0001 C CNN
	1    1100 1350
	-1   0    0    1   
$EndComp
Wire Wire Line
	1100 1300 1100 1350
Wire Wire Line
	1100 1850 1100 1900
$Comp
L power:+5V #PWR03
U 1 1 5C20B0FA
P 1550 1350
F 0 "#PWR03" H 1550 1200 50  0001 C CNN
F 1 "+5V" H 1565 1523 50  0000 C CNN
F 2 "" H 1550 1350 50  0001 C CNN
F 3 "" H 1550 1350 50  0001 C CNN
	1    1550 1350
	-1   0    0    1   
$EndComp
$Comp
L power:PWR_FLAG #FLG03
U 1 1 5C20B116
P 1550 1300
F 0 "#FLG03" H 1550 1375 50  0001 C CNN
F 1 "PWR_FLAG" H 1550 1474 50  0000 C CNN
F 2 "" H 1550 1300 50  0001 C CNN
F 3 "~" H 1550 1300 50  0001 C CNN
	1    1550 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	1550 1300 1550 1350
$Comp
L power:+12V #PWR07
U 1 1 5C20B221
P 2200 1350
F 0 "#PWR07" H 2200 1200 50  0001 C CNN
F 1 "+12V" H 2215 1523 50  0000 C CNN
F 2 "" H 2200 1350 50  0001 C CNN
F 3 "" H 2200 1350 50  0001 C CNN
	1    2200 1350
	-1   0    0    1   
$EndComp
$Comp
L power:GNDD #PWR08
U 1 1 5C20B289
P 2200 1900
F 0 "#PWR08" H 2200 1650 50  0001 C CNN
F 1 "GNDD" H 2204 1745 50  0000 C CNN
F 2 "" H 2200 1900 50  0001 C CNN
F 3 "" H 2200 1900 50  0001 C CNN
	1    2200 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	2200 1300 2200 1350
Wire Wire Line
	2200 1850 2200 1900
$Comp
L power:GND #PWR05
U 1 1 5C20D6E8
P 2050 4950
F 0 "#PWR05" H 2050 4700 50  0001 C CNN
F 1 "GND" H 2055 4777 50  0000 C CNN
F 2 "" H 2050 4950 50  0001 C CNN
F 3 "" H 2050 4950 50  0001 C CNN
	1    2050 4950
	1    0    0    -1  
$EndComp
Wire Wire Line
	1950 4850 1950 4900
Wire Wire Line
	1950 4900 2050 4900
Wire Wire Line
	2050 4900 2050 4950
Wire Wire Line
	2050 4900 2050 4850
Connection ~ 2050 4900
NoConn ~ 2450 3250
NoConn ~ 2450 3350
$Comp
L power:+9V #PWR04
U 1 1 5C20D9BA
P 1850 2800
F 0 "#PWR04" H 1850 2650 50  0001 C CNN
F 1 "+9V" H 1865 2973 50  0000 C CNN
F 2 "" H 1850 2800 50  0001 C CNN
F 3 "" H 1850 2800 50  0001 C CNN
	1    1850 2800
	1    0    0    -1  
$EndComp
NoConn ~ 2050 2850
$Comp
L power:+5V #PWR06
U 1 1 5C20DA4C
P 2150 2800
F 0 "#PWR06" H 2150 2650 50  0001 C CNN
F 1 "+5V" H 2165 2973 50  0000 C CNN
F 2 "" H 2150 2800 50  0001 C CNN
F 3 "" H 2150 2800 50  0001 C CNN
	1    2150 2800
	1    0    0    -1  
$EndComp
Wire Wire Line
	1850 2800 1850 2850
Wire Wire Line
	2150 2850 2150 2800
NoConn ~ 1450 4350
NoConn ~ 1450 4450
NoConn ~ 1450 4550
NoConn ~ 2450 3650
NoConn ~ 2450 3850
NoConn ~ 2450 3950
NoConn ~ 2450 4050
NoConn ~ 2450 4150
NoConn ~ 2450 4450
NoConn ~ 2450 4550
Text GLabel 2550 4250 2    50   Input ~ 0
SDA
Text GLabel 2550 4350 2    50   Input ~ 0
SCL
Wire Wire Line
	2450 4250 2550 4250
Wire Wire Line
	2550 4350 2450 4350
$Comp
L Connector:Barrel_Jack_Switch J1
U 1 1 5C20F30A
P 3000 1300
F 0 "J1" H 3055 1625 50  0000 C CNN
F 1 "Barrel_Jack" H 3055 1534 50  0000 C CNN
F 2 "Connector_BarrelJack:BarrelJack_CUI_PJ-102AH_Horizontal" H 3050 1260 50  0001 C CNN
F 3 "~" H 3050 1260 50  0001 C CNN
	1    3000 1300
	1    0    0    -1  
$EndComp
$Comp
L Connector:Barrel_Jack_Switch J2
U 1 1 5C20F358
P 3000 1900
F 0 "J2" H 3055 2225 50  0000 C CNN
F 1 "Barrel_Jack" H 3055 2134 50  0000 C CNN
F 2 "Connector_BarrelJack:BarrelJack_CUI_PJ-102AH_Horizontal" H 3050 1860 50  0001 C CNN
F 3 "~" H 3050 1860 50  0001 C CNN
	1    3000 1900
	1    0    0    -1  
$EndComp
$Comp
L power:+12V #PWR011
U 1 1 5C20F4BC
P 3400 1800
F 0 "#PWR011" H 3400 1650 50  0001 C CNN
F 1 "+12V" V 3415 1928 50  0000 L CNN
F 2 "" H 3400 1800 50  0001 C CNN
F 3 "" H 3400 1800 50  0001 C CNN
	1    3400 1800
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR010
U 1 1 5C20F52D
P 3400 1400
F 0 "#PWR010" H 3400 1150 50  0001 C CNN
F 1 "GND" V 3405 1272 50  0000 R CNN
F 2 "" H 3400 1400 50  0001 C CNN
F 3 "" H 3400 1400 50  0001 C CNN
	1    3400 1400
	0    -1   -1   0   
$EndComp
$Comp
L power:+9V #PWR09
U 1 1 5C20F602
P 3400 1200
F 0 "#PWR09" H 3400 1050 50  0001 C CNN
F 1 "+9V" V 3415 1328 50  0000 L CNN
F 2 "" H 3400 1200 50  0001 C CNN
F 3 "" H 3400 1200 50  0001 C CNN
	1    3400 1200
	0    1    1    0   
$EndComp
$Comp
L power:GNDD #PWR012
U 1 1 5C20F688
P 3400 2000
F 0 "#PWR012" H 3400 1750 50  0001 C CNN
F 1 "GNDD" V 3404 1890 50  0000 R CNN
F 2 "" H 3400 2000 50  0001 C CNN
F 3 "" H 3400 2000 50  0001 C CNN
	1    3400 2000
	0    -1   -1   0   
$EndComp
Wire Wire Line
	3300 1200 3400 1200
Wire Wire Line
	3400 1400 3350 1400
Wire Wire Line
	3300 1800 3400 1800
Wire Wire Line
	3400 2000 3350 2000
Wire Notes Line
	800  2400 800  5350
Wire Notes Line
	800  850  800  2300
Text Notes 850  1000 0    50   ~ 0
Power
$Sheet
S 1400 5650 950  700 
U 5C211542
F0 "Input 1" 50
F1 "input.sch" 50
F2 "A0" I L 1400 5750 50 
F3 "A1" I L 1400 5850 50 
F4 "A2" I L 1400 5950 50 
F5 "INTA" I L 1400 6050 50 
F6 "INTB" I L 1400 6150 50 
F7 "~RESET" I L 1400 6250 50 
$EndSheet
$Sheet
S 1400 6600 950  700 
U 5C211551
F0 "Input 2" 50
F1 "input.sch" 50
F2 "A0" I L 1400 6700 50 
F3 "A1" I L 1400 6800 50 
F4 "A2" I L 1400 6900 50 
F5 "INTA" I L 1400 7000 50 
F6 "INTB" I L 1400 7100 50 
F7 "~RESET" I L 1400 7200 50 
$EndSheet
$Comp
L power:GND #PWR019
U 1 1 5C24134C
P 1250 7300
F 0 "#PWR019" H 1250 7050 50  0001 C CNN
F 1 "GND" H 1255 7127 50  0000 C CNN
F 2 "" H 1250 7300 50  0001 C CNN
F 3 "" H 1250 7300 50  0001 C CNN
	1    1250 7300
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR018
U 1 1 5C2413B9
P 1150 5700
F 0 "#PWR018" H 1150 5550 50  0001 C CNN
F 1 "+5V" H 1165 5873 50  0000 C CNN
F 2 "" H 1150 5700 50  0001 C CNN
F 3 "" H 1150 5700 50  0001 C CNN
	1    1150 5700
	1    0    0    -1  
$EndComp
Wire Wire Line
	1400 5750 1250 5750
Wire Wire Line
	1250 5750 1250 5850
Wire Wire Line
	1400 5850 1250 5850
Connection ~ 1250 5850
Wire Wire Line
	1250 5850 1250 5950
Wire Wire Line
	1400 5950 1250 5950
Wire Wire Line
	1400 6800 1250 6800
Wire Wire Line
	1250 6800 1250 6900
Wire Wire Line
	1400 6900 1250 6900
Text Notes 1400 5550 0    50   ~ 0
0x20
Text Notes 1400 6500 0    50   ~ 0
0x21
$Sheet
S 2600 5850 950  500 
U 5C242719
F0 "Output 1" 50
F1 "output.sch" 50
F2 "A0" I R 3550 5950 50 
F3 "A1" I R 3550 6050 50 
F4 "A2" I R 3550 6150 50 
F5 "~RESET" I R 3550 6250 50 
$EndSheet
$Sheet
S 2600 6600 950  500 
U 5C24272E
F0 "Output 2" 50
F1 "output.sch" 50
F2 "A0" I R 3550 6700 50 
F3 "A1" I R 3550 6800 50 
F4 "A2" I R 3550 6900 50 
F5 "~RESET" I R 3550 7000 50 
$EndSheet
$Comp
L power:GND #PWR020
U 1 1 5C2430F8
P 3750 7250
F 0 "#PWR020" H 3750 7000 50  0001 C CNN
F 1 "GND" H 3755 7077 50  0000 C CNN
F 2 "" H 3750 7250 50  0001 C CNN
F 3 "" H 3750 7250 50  0001 C CNN
	1    3750 7250
	1    0    0    -1  
$EndComp
Wire Wire Line
	3550 6150 3750 6150
Wire Wire Line
	3750 6150 3750 6900
Wire Wire Line
	3550 6900 3750 6900
Connection ~ 3750 6900
Wire Wire Line
	3750 6900 3750 7250
Wire Wire Line
	3550 5950 3750 5950
Wire Wire Line
	3750 5950 3750 6150
Connection ~ 3750 6150
$Comp
L power:+5V #PWR021
U 1 1 5C243E0C
P 3950 5700
F 0 "#PWR021" H 3950 5550 50  0001 C CNN
F 1 "+5V" H 3965 5873 50  0000 C CNN
F 2 "" H 3950 5700 50  0001 C CNN
F 3 "" H 3950 5700 50  0001 C CNN
	1    3950 5700
	1    0    0    -1  
$EndComp
Wire Wire Line
	3950 5700 3950 6050
Wire Wire Line
	3950 6050 3550 6050
Wire Wire Line
	3950 6050 3950 6700
Wire Wire Line
	3950 6700 3550 6700
Connection ~ 3950 6050
Wire Wire Line
	3550 6800 3950 6800
Wire Wire Line
	3950 6800 3950 6700
Connection ~ 3950 6700
$Comp
L Connector:Screw_Terminal_01x02 J4
U 1 1 5C2CE32C
P 3900 1800
F 0 "J4" H 3820 2017 50  0000 C CNN
F 1 "Screw_Terminal_01x02" H 3820 1926 50  0000 C CNN
F 2 "TerminalBlock_TE-Connectivity:TerminalBlock_TE_282834-2_1x02_P2.54mm_Horizontal" H 3900 1800 50  0001 C CNN
F 3 "~" H 3900 1800 50  0001 C CNN
	1    3900 1800
	-1   0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x03 J3
U 1 1 5C2CE585
P 3850 1300
F 0 "J3" H 3770 1617 50  0000 C CNN
F 1 "Screw_Terminal_01x03" H 3770 1526 50  0000 C CNN
F 2 "TerminalBlock_TE-Connectivity:TerminalBlock_TE_282834-3_1x03_P2.54mm_Horizontal" H 3850 1300 50  0001 C CNN
F 3 "~" H 3850 1300 50  0001 C CNN
	1    3850 1300
	-1   0    0    -1  
$EndComp
$Comp
L power:+9V #PWR013
U 1 1 5C2CE665
P 4150 1200
F 0 "#PWR013" H 4150 1050 50  0001 C CNN
F 1 "+9V" V 4165 1328 50  0000 L CNN
F 2 "" H 4150 1200 50  0001 C CNN
F 3 "" H 4150 1200 50  0001 C CNN
	1    4150 1200
	0    1    1    0   
$EndComp
$Comp
L power:+5V #PWR014
U 1 1 5C2CE6DE
P 4150 1300
F 0 "#PWR014" H 4150 1150 50  0001 C CNN
F 1 "+5V" V 4165 1428 50  0000 L CNN
F 2 "" H 4150 1300 50  0001 C CNN
F 3 "" H 4150 1300 50  0001 C CNN
	1    4150 1300
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR015
U 1 1 5C2CE757
P 4150 1400
F 0 "#PWR015" H 4150 1150 50  0001 C CNN
F 1 "GND" V 4155 1272 50  0000 R CNN
F 2 "" H 4150 1400 50  0001 C CNN
F 3 "" H 4150 1400 50  0001 C CNN
	1    4150 1400
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4050 1200 4150 1200
Wire Wire Line
	4150 1300 4050 1300
Wire Wire Line
	4050 1400 4150 1400
Wire Notes Line
	4550 850  4550 2300
Wire Notes Line
	800  2300 4550 2300
Wire Notes Line
	800  850  4550 850 
$Comp
L power:+12V #PWR016
U 1 1 5C2D0559
P 4150 1800
F 0 "#PWR016" H 4150 1650 50  0001 C CNN
F 1 "+12V" V 4165 1928 50  0000 L CNN
F 2 "" H 4150 1800 50  0001 C CNN
F 3 "" H 4150 1800 50  0001 C CNN
	1    4150 1800
	0    1    1    0   
$EndComp
$Comp
L power:GNDD #PWR017
U 1 1 5C2D05FC
P 4150 1900
F 0 "#PWR017" H 4150 1650 50  0001 C CNN
F 1 "GNDD" V 4154 1790 50  0000 R CNN
F 2 "" H 4150 1900 50  0001 C CNN
F 3 "" H 4150 1900 50  0001 C CNN
	1    4150 1900
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4100 1800 4150 1800
Wire Wire Line
	4150 1900 4100 1900
$Comp
L Mechanical:MountingHole H1
U 1 1 5C2D1997
P 4900 1250
F 0 "H1" H 5000 1296 50  0000 L CNN
F 1 "MountingHole" H 5000 1205 50  0000 L CNN
F 2 "MountingHole:MountingHole_4.3mm_M4" H 4900 1250 50  0001 C CNN
F 3 "~" H 4900 1250 50  0001 C CNN
	1    4900 1250
	1    0    0    -1  
$EndComp
$Comp
L Mechanical:MountingHole H2
U 1 1 5C2D19E7
P 4900 1500
F 0 "H2" H 5000 1546 50  0000 L CNN
F 1 "MountingHole" H 5000 1455 50  0000 L CNN
F 2 "MountingHole:MountingHole_4.3mm_M4" H 4900 1500 50  0001 C CNN
F 3 "~" H 4900 1500 50  0001 C CNN
	1    4900 1500
	1    0    0    -1  
$EndComp
$Comp
L Mechanical:MountingHole H3
U 1 1 5C2D1A0D
P 4900 1750
F 0 "H3" H 5000 1796 50  0000 L CNN
F 1 "MountingHole" H 5000 1705 50  0000 L CNN
F 2 "MountingHole:MountingHole_4.3mm_M4" H 4900 1750 50  0001 C CNN
F 3 "~" H 4900 1750 50  0001 C CNN
	1    4900 1750
	1    0    0    -1  
$EndComp
$Comp
L Mechanical:MountingHole H4
U 1 1 5C2D1A41
P 4900 2000
F 0 "H4" H 5000 2046 50  0000 L CNN
F 1 "MountingHole" H 5000 1955 50  0000 L CNN
F 2 "MountingHole:MountingHole_4.3mm_M4" H 4900 2000 50  0001 C CNN
F 3 "~" H 4900 2000 50  0001 C CNN
	1    4900 2000
	1    0    0    -1  
$EndComp
Wire Notes Line
	4650 850  4650 2300
Wire Notes Line
	4650 2300 5950 2300
Wire Notes Line
	5950 2300 5950 850 
Wire Notes Line
	5950 850  4650 850 
Text Notes 4700 1050 0    50   ~ 0
Mounting\nholes
$Comp
L MCU_Module:Arduino_Nano_v3.x A1
U 1 1 5C20D40E
P 1950 3850
F 0 "A1" H 1600 4800 50  0000 C CNN
F 1 "Arduino_Nano_v3.x" H 2500 2850 50  0000 C CNN
F 2 "Module:Arduino_Nano" H 2100 2900 50  0001 L CNN
F 3 "http://www.mouser.com/pdfdocs/Gravitech_Arduino_Nano3_0.pdf" H 1950 2850 50  0001 C CNN
	1    1950 3850
	1    0    0    -1  
$EndComp
Wire Wire Line
	1150 5700 1150 6700
Wire Wire Line
	800  6050 1400 6050
Wire Wire Line
	800  6150 1400 6150
Wire Wire Line
	800  7100 1400 7100
Text Label 800  6050 0    50   ~ 0
INTA1
Text Label 800  6150 0    50   ~ 0
INTB1
Text Label 800  7000 0    50   ~ 0
INTA2
Text Label 800  7100 0    50   ~ 0
INTB2
Text Label 950  3450 0    50   ~ 0
INTA1
Text Label 950  3550 0    50   ~ 0
INTA2
Text Label 950  3650 0    50   ~ 0
INTB1
Text Label 950  3750 0    50   ~ 0
INTB2
Wire Wire Line
	800  6250 1400 6250
Text Label 800  6250 0    50   ~ 0
~RESET_IN1
Wire Wire Line
	800  7000 1400 7000
Wire Wire Line
	1250 7300 1250 6900
Connection ~ 1250 6900
Wire Wire Line
	1150 6700 1400 6700
Wire Wire Line
	1250 5950 1250 6800
Connection ~ 1250 5950
Connection ~ 1250 6800
Wire Wire Line
	1400 7200 800  7200
Text Label 800  7200 0    50   ~ 0
~RESET_IN2
Wire Wire Line
	3550 6250 4250 6250
Wire Wire Line
	3550 7000 4250 7000
Text Label 4250 6250 0    50   ~ 0
~RESET_OUT1
Text Label 4250 7000 0    50   ~ 0
~RESET_OUT2
Text Label 950  3850 0    50   ~ 0
~RESET_IN1
Text Label 950  4150 0    50   ~ 0
~RESET_IN2
Text Label 950  4050 0    50   ~ 0
~RESET_OUT1
Text Label 950  3950 0    50   ~ 0
~RESET_OUT2
Wire Wire Line
	950  3450 1450 3450
Wire Wire Line
	950  3550 1450 3550
Wire Wire Line
	950  3650 1450 3650
Wire Wire Line
	950  3750 1450 3750
Wire Wire Line
	950  3850 1450 3850
Wire Wire Line
	950  3950 1450 3950
Wire Wire Line
	950  4050 1450 4050
Wire Wire Line
	950  4150 1450 4150
Wire Wire Line
	1450 3250 950  3250
Wire Wire Line
	950  3350 1450 3350
Text Label 950  3250 0    50   ~ 0
RX
Text Label 950  3350 0    50   ~ 0
TX
Text Notes 2450 2500 0    50   ~ 0
Arduino
Text Notes 9150 1000 0    50   ~ 0
Serial communication
Wire Wire Line
	7800 1450 8150 1450
Wire Wire Line
	7800 1650 8150 1650
Wire Wire Line
	7800 1550 8150 1550
Wire Wire Line
	7800 1250 8150 1250
Text Label 8150 1450 0    50   ~ 0
Vcomm
Text Label 8150 1250 0    50   ~ 0
GNDcomm
Text Label 8150 1650 0    50   ~ 0
TXout
Text Label 8150 1550 0    50   ~ 0
RXout
Text Notes 2600 5750 0    50   ~ 0
0x22
Text Notes 2600 6500 0    50   ~ 0
0x23
Wire Notes Line
	800  5350 2900 5350
Wire Notes Line
	2900 5350 2900 2400
Wire Notes Line
	2900 2400 800  2400
Wire Wire Line
	3300 1300 3350 1300
Wire Wire Line
	3350 1300 3350 1400
Connection ~ 3350 1400
Wire Wire Line
	3350 1400 3300 1400
Wire Wire Line
	3300 1900 3350 1900
Wire Wire Line
	3350 1900 3350 2000
Connection ~ 3350 2000
Wire Wire Line
	3350 2000 3300 2000
Wire Wire Line
	4000 3400 4150 3400
Wire Wire Line
	4150 3400 4150 3600
$Comp
L power:GND #PWR0102
U 1 1 5C298323
P 4150 3600
F 0 "#PWR0102" H 4150 3350 50  0001 C CNN
F 1 "GND" H 4155 3427 50  0000 C CNN
F 2 "" H 4150 3600 50  0001 C CNN
F 3 "" H 4150 3600 50  0001 C CNN
	1    4150 3600
	1    0    0    -1  
$EndComp
Text Label 3150 3300 0    50   ~ 0
PRG_MODE
Wire Wire Line
	950  4250 1450 4250
Text Label 950  4250 0    50   ~ 0
PRG_MODE
Wire Wire Line
	3150 3300 3600 3300
$Comp
L Switch:SW_SPDT SW1
U 1 1 5C2A8AB1
P 3800 3300
F 0 "SW1" H 3800 3585 50  0000 C CNN
F 1 "SW_SPDT" H 3800 3494 50  0000 C CNN
F 2 "digikey-footprints:Switch_Slide_11.6x4mm_EG1218" H 3800 3300 50  0001 C CNN
F 3 "" H 3800 3300 50  0001 C CNN
	1    3800 3300
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR0101
U 1 1 5C2AB353
P 4150 2950
F 0 "#PWR0101" H 4150 2800 50  0001 C CNN
F 1 "+5V" H 4165 3123 50  0000 C CNN
F 2 "" H 4150 2950 50  0001 C CNN
F 3 "" H 4150 2950 50  0001 C CNN
	1    4150 2950
	1    0    0    -1  
$EndComp
Wire Wire Line
	4000 3200 4150 3200
Wire Wire Line
	4150 3200 4150 2950
$Comp
L Connector_Generic:Conn_02x05_Odd_Even J5
U 1 1 5C29028D
P 6750 1450
F 0 "J5" H 6800 1867 50  0000 C CNN
F 1 "Conn_02x05_Odd_Even" H 6800 1776 50  0000 C CNN
F 2 "Connector_IDC:IDC-Header_2x05_P2.54mm_Vertical" H 6750 1450 50  0001 C CNN
F 3 "~" H 6750 1450 50  0001 C CNN
	1    6750 1450
	1    0    0    -1  
$EndComp
Wire Wire Line
	6550 1250 6200 1250
Text Label 6200 1250 0    50   ~ 0
Vcomm
Wire Wire Line
	6550 1650 6200 1650
Text Label 6200 1650 0    50   ~ 0
GNDcomm
Text Label 7400 1450 2    50   ~ 0
GNDcomm
Wire Wire Line
	7050 1450 7400 1450
NoConn ~ 6550 1350
NoConn ~ 6550 1450
NoConn ~ 6550 1550
NoConn ~ 7050 1250
NoConn ~ 7050 1350
Wire Wire Line
	7050 1550 7400 1550
Wire Wire Line
	7400 1650 7050 1650
Text Label 7400 1550 2    50   ~ 0
RXout
Text Label 7400 1650 2    50   ~ 0
TXout
$Comp
L Connector:Conn_01x06_Male J6
U 1 1 5C28F19F
P 7600 1450
F 0 "J6" H 7706 1828 50  0000 C CNN
F 1 "Conn_01x06_Male" H 7706 1737 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x06_P2.54mm_Horizontal" H 7600 1450 50  0001 C CNN
F 3 "~" H 7600 1450 50  0001 C CNN
	1    7600 1450
	1    0    0    -1  
$EndComp
NoConn ~ 7800 1350
NoConn ~ 7800 1750
Wire Notes Line
	6050 850  10050 850 
$Comp
L pcb:CPC5001G U1
U 1 1 5C29FFF7
P 8400 2400
F 0 "U1" H 8400 2815 50  0000 C CNN
F 1 "CPC5001G" H 8400 2724 50  0000 C CNN
F 2 "Package_DIP:DIP-8_W7.62mm" H 8400 2300 50  0001 C CNN
F 3 "" H 8400 2300 50  0001 C CNN
	1    8400 2400
	1    0    0    -1  
$EndComp
Wire Wire Line
	8000 2550 7650 2550
Wire Wire Line
	9300 2250 9150 2250
Wire Wire Line
	8800 2450 9150 2450
Text Label 7200 2350 0    50   ~ 0
GNDcomm
Text Label 7200 2550 0    50   ~ 0
Vcomm
Connection ~ 7650 2550
Wire Wire Line
	7200 2550 7650 2550
Wire Wire Line
	9300 2250 9300 2150
$Comp
L power:+5V #PWR0103
U 1 1 5C2B109E
P 9300 2150
F 0 "#PWR0103" H 9300 2000 50  0001 C CNN
F 1 "+5V" H 9315 2323 50  0000 C CNN
F 2 "" H 9300 2150 50  0001 C CNN
F 3 "" H 9300 2150 50  0001 C CNN
	1    9300 2150
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0104
U 1 1 5C2B115D
P 9300 2550
F 0 "#PWR0104" H 9300 2300 50  0001 C CNN
F 1 "GND" H 9305 2377 50  0000 C CNN
F 2 "" H 9300 2550 50  0001 C CNN
F 3 "" H 9300 2550 50  0001 C CNN
	1    9300 2550
	1    0    0    -1  
$EndComp
Wire Wire Line
	9300 2450 9300 2550
Wire Wire Line
	8800 2350 8850 2350
Text Label 9100 2350 2    50   ~ 0
RX
Wire Wire Line
	8850 2350 8850 2150
Connection ~ 8850 2350
Wire Wire Line
	8850 2350 9100 2350
$Comp
L Device:R R66
U 1 1 5C2B8ED9
P 8850 2000
F 0 "R66" H 8920 2046 50  0000 L CNN
F 1 "1k" H 8920 1955 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P7.62mm_Horizontal" V 8780 2000 50  0001 C CNN
F 3 "~" H 8850 2000 50  0001 C CNN
	1    8850 2000
	1    0    0    -1  
$EndComp
Wire Wire Line
	8850 1850 8850 1800
Wire Wire Line
	8850 1800 9150 1800
Wire Wire Line
	9150 1800 9150 2250
Connection ~ 9150 2250
Wire Wire Line
	9150 2250 8800 2250
Wire Wire Line
	8800 2550 9100 2550
Text Label 9100 2550 2    50   ~ 0
TX
Wire Wire Line
	8000 2250 7650 2250
Text Label 7650 2250 0    50   ~ 0
RXout
Wire Wire Line
	8000 2450 7950 2450
Wire Wire Line
	7950 2450 7950 2850
Wire Wire Line
	7950 2850 7650 2850
$Comp
L Device:R R65
U 1 1 5C2C4D2C
P 7650 2700
F 0 "R65" H 7720 2746 50  0000 L CNN
F 1 "1k" H 7720 2655 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P7.62mm_Horizontal" V 7580 2700 50  0001 C CNN
F 3 "~" H 7650 2700 50  0001 C CNN
	1    7650 2700
	1    0    0    -1  
$EndComp
Connection ~ 7650 2850
Wire Wire Line
	7650 2850 7200 2850
Text Label 7200 2850 0    50   ~ 0
TXout
Wire Wire Line
	7200 2350 7650 2350
$Comp
L Device:C_Small C1
U 1 1 5C2EE7BD
P 7650 2450
F 0 "C1" H 7742 2496 50  0000 L CNN
F 1 "0.1uF" H 7742 2405 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 7650 2450 50  0001 C CNN
F 3 "~" H 7650 2450 50  0001 C CNN
	1    7650 2450
	1    0    0    -1  
$EndComp
Connection ~ 7650 2350
Wire Wire Line
	7650 2350 8000 2350
$Comp
L Device:C_Small C2
U 1 1 5C2EE82F
P 9150 2350
F 0 "C2" H 9242 2396 50  0000 L CNN
F 1 "0.1uF" H 9242 2305 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 9150 2350 50  0001 C CNN
F 3 "~" H 9150 2350 50  0001 C CNN
	1    9150 2350
	1    0    0    -1  
$EndComp
Connection ~ 9150 2450
Wire Wire Line
	9150 2450 9300 2450
Wire Notes Line
	6050 850  6050 3100
Wire Notes Line
	6050 3100 10050 3100
Wire Notes Line
	10050 3100 10050 850 
$EndSCHEMATC