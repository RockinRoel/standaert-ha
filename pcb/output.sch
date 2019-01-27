EESchema Schematic File Version 4
LIBS:pcb-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 12 45
Title "Standaert Home Automation"
Date ""
Rev "1"
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 "Author: Roel Standaert"
$EndDescr
$Comp
L dk_Interface-I-O-Expanders:MCP23017-E_SP U12
U 1 1 5C242C47
P 5550 4100
AR Path="/5C242719/5C242C47" Ref="U12"  Part="1" 
AR Path="/5C24272E/5C242C47" Ref="U13"  Part="1" 
F 0 "U13" H 5650 5100 60  0000 C CNN
F 1 "MCP23017-E_SP" H 5650 4994 60  0000 C CNN
F 2 "Package_DIP:DIP-28_W7.62mm" H 5750 4300 60  0001 L CNN
F 3 "http://www.microchip.com/mymicrochip/filehandler.aspx?ddocname=en023709" H 5750 4400 60  0001 L CNN
F 4 "MCP23017-E/SP-ND" H 5750 4500 60  0001 L CNN "Digi-Key_PN"
F 5 "MCP23017-E/SP" H 5750 4600 60  0001 L CNN "MPN"
F 6 "Integrated Circuits (ICs)" H 5750 4700 60  0001 L CNN "Category"
F 7 "Interface - I/O Expanders" H 5750 4800 60  0001 L CNN "Family"
F 8 "http://www.microchip.com/mymicrochip/filehandler.aspx?ddocname=en023709" H 5750 4900 60  0001 L CNN "DK_Datasheet_Link"
F 9 "/product-detail/en/microchip-technology/MCP23017-E-SP/MCP23017-E-SP-ND/894272" H 5750 5000 60  0001 L CNN "DK_Detail_Page"
F 10 "IC I/O EXPANDER I2C 16B 28SDIP" H 5750 5100 60  0001 L CNN "Description"
F 11 "Microchip Technology" H 5750 5200 60  0001 L CNN "Manufacturer"
F 12 "Active" H 5750 5300 60  0001 L CNN "Status"
	1    5550 4100
	1    0    0    -1  
$EndComp
Wire Wire Line
	5650 3300 5650 3250
Wire Wire Line
	5650 3250 6100 3250
$Comp
L power:+5V #PWR047
U 1 1 5C242D4E
P 6100 3150
AR Path="/5C242719/5C242D4E" Ref="#PWR047"  Part="1" 
AR Path="/5C24272E/5C242D4E" Ref="#PWR066"  Part="1" 
F 0 "#PWR066" H 6100 3000 50  0001 C CNN
F 1 "+5V" H 6115 3323 50  0000 C CNN
F 2 "" H 6100 3150 50  0001 C CNN
F 3 "" H 6100 3150 50  0001 C CNN
	1    6100 3150
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 3150 6100 3250
Connection ~ 6100 3250
NoConn ~ 6050 3800
NoConn ~ 6050 3900
$Comp
L power:GND #PWR046
U 1 1 5C242DFE
P 5650 5800
AR Path="/5C242719/5C242DFE" Ref="#PWR046"  Part="1" 
AR Path="/5C24272E/5C242DFE" Ref="#PWR065"  Part="1" 
F 0 "#PWR065" H 5650 5550 50  0001 C CNN
F 1 "GND" H 5655 5627 50  0000 C CNN
F 2 "" H 5650 5800 50  0001 C CNN
F 3 "" H 5650 5800 50  0001 C CNN
	1    5650 5800
	1    0    0    -1  
$EndComp
Wire Wire Line
	5650 5700 5650 5800
Text HLabel 5150 5300 0    50   Input ~ 0
A0
Text HLabel 5150 5400 0    50   Input ~ 0
A1
Text HLabel 5150 5500 0    50   Input ~ 0
A2
Wire Wire Line
	5150 5300 5250 5300
Wire Wire Line
	5250 5400 5150 5400
Wire Wire Line
	5150 5500 5250 5500
Text GLabel 5100 5100 0    50   Input ~ 0
SDA
Text GLabel 5100 5200 0    50   Input ~ 0
SCL
Wire Wire Line
	5100 5100 5250 5100
Wire Wire Line
	5100 5200 5250 5200
$Sheet
S 1750 1550 500  300 
U 5C245432
F0 "OutWithLED 1" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 1700 50 
F3 "Out" I L 1750 1700 50 
$EndSheet
$Sheet
S 1750 2050 500  300 
U 5C2457B9
F0 "OutWithLED 2" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 2200 50 
F3 "Out" I L 1750 2200 50 
$EndSheet
$Sheet
S 1750 2550 500  300 
U 5C2458E1
F0 "OutWithLED 3" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 2700 50 
F3 "Out" I L 1750 2700 50 
$EndSheet
$Sheet
S 1750 3050 500  300 
U 5C2458E5
F0 "OutWithLED 4" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 3200 50 
F3 "Out" I L 1750 3200 50 
$EndSheet
$Sheet
S 1750 3550 500  300 
U 5C245AA4
F0 "OutWithLED 5" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 3700 50 
F3 "Out" I L 1750 3700 50 
$EndSheet
$Sheet
S 1750 4050 500  300 
U 5C245AA8
F0 "OutWithLED 6" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 4200 50 
F3 "Out" I L 1750 4200 50 
$EndSheet
$Sheet
S 1750 4550 500  300 
U 5C245AAC
F0 "OutWithLED 7" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 4700 50 
F3 "Out" I L 1750 4700 50 
$EndSheet
$Sheet
S 1750 5050 500  300 
U 5C245AB0
F0 "OutWithLED 8" 50
F1 "out_with_led.sch" 50
F2 "In" I R 2250 5200 50 
F3 "Out" I L 1750 5200 50 
$EndSheet
$Sheet
S 3350 1550 500  300 
U 5C245CF0
F0 "OutWithLED 9" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 1700 50 
F3 "Out" I L 3350 1700 50 
$EndSheet
$Sheet
S 3350 2050 500  300 
U 5C245CF4
F0 "OutWithLED 10" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 2200 50 
F3 "Out" I L 3350 2200 50 
$EndSheet
$Sheet
S 3350 2550 500  300 
U 5C245CF8
F0 "OutWithLED 11" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 2700 50 
F3 "Out" I L 3350 2700 50 
$EndSheet
$Sheet
S 3350 3050 500  300 
U 5C245CFC
F0 "OutWithLED 12" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 3200 50 
F3 "Out" I L 3350 3200 50 
$EndSheet
$Sheet
S 3350 3550 500  300 
U 5C245D00
F0 "OutWithLED 13" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 3700 50 
F3 "Out" I L 3350 3700 50 
$EndSheet
$Sheet
S 3350 4050 500  300 
U 5C245D04
F0 "OutWithLED 14" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 4200 50 
F3 "Out" I L 3350 4200 50 
$EndSheet
$Sheet
S 3350 4550 500  300 
U 5C245D08
F0 "OutWithLED 15" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 4700 50 
F3 "Out" I L 3350 4700 50 
$EndSheet
$Sheet
S 3350 5050 500  300 
U 5C245D0C
F0 "OutWithLED 16" 50
F1 "out_with_led.sch" 50
F2 "In" I R 3850 5200 50 
F3 "Out" I L 3350 5200 50 
$EndSheet
Wire Wire Line
	2250 1700 2600 1700
Wire Wire Line
	2250 2200 2600 2200
Wire Wire Line
	2250 2700 2600 2700
Wire Wire Line
	2250 3200 2600 3200
Wire Wire Line
	2250 3700 2600 3700
Wire Wire Line
	2250 4200 2600 4200
Wire Wire Line
	2250 4700 2600 4700
Wire Wire Line
	2250 5200 2600 5200
Entry Wire Line
	2600 1700 2700 1800
Entry Wire Line
	2600 2200 2700 2300
Entry Wire Line
	2600 2700 2700 2800
Entry Wire Line
	2600 3200 2700 3300
Entry Wire Line
	2600 3700 2700 3800
Entry Wire Line
	2600 4200 2700 4300
Entry Wire Line
	2600 4700 2700 4800
Entry Wire Line
	2600 5200 2700 5300
Wire Wire Line
	3850 1700 4200 1700
Wire Wire Line
	3850 2200 4200 2200
Wire Wire Line
	3850 2700 4200 2700
Wire Wire Line
	3850 3200 4200 3200
Wire Wire Line
	3850 3700 4200 3700
Wire Wire Line
	3850 4200 4200 4200
Wire Wire Line
	3850 4700 4200 4700
Wire Wire Line
	3850 5200 4200 5200
Entry Wire Line
	4200 1700 4300 1800
Entry Wire Line
	4200 2200 4300 2300
Entry Wire Line
	4200 2700 4300 2800
Entry Wire Line
	4200 3200 4300 3300
Entry Wire Line
	4200 3700 4300 3800
Entry Wire Line
	4200 4200 4300 4300
Entry Wire Line
	4200 4700 4300 4800
Entry Wire Line
	4200 5200 4300 5300
Wire Bus Line
	4300 1800 4800 1800
Entry Wire Line
	4800 3400 4900 3500
Entry Wire Line
	4800 3500 4900 3600
Entry Wire Line
	4800 3600 4900 3700
Entry Wire Line
	4800 3700 4900 3800
Entry Wire Line
	4800 3800 4900 3900
Entry Wire Line
	4800 3900 4900 4000
Entry Wire Line
	4800 4000 4900 4100
Entry Wire Line
	4800 4100 4900 4200
Entry Wire Line
	4800 4200 4900 4300
Entry Wire Line
	4800 4300 4900 4400
Entry Wire Line
	4800 4400 4900 4500
Entry Wire Line
	4800 4500 4900 4600
Entry Wire Line
	4800 4600 4900 4700
Entry Wire Line
	4800 4700 4900 4800
Entry Wire Line
	4800 4800 4900 4900
Wire Wire Line
	4900 3500 5250 3500
Wire Wire Line
	5250 3600 4900 3600
Wire Wire Line
	4900 3700 5250 3700
Wire Wire Line
	5250 3800 4900 3800
Wire Wire Line
	4900 3900 5250 3900
Wire Wire Line
	5250 4000 4900 4000
Wire Wire Line
	4900 4100 5250 4100
Wire Wire Line
	5250 4200 4900 4200
Wire Wire Line
	4900 4300 5250 4300
Wire Wire Line
	4900 4400 5250 4400
Wire Wire Line
	5250 4500 4900 4500
Wire Wire Line
	4900 4600 5250 4600
Wire Wire Line
	5250 4700 4900 4700
Wire Wire Line
	4900 4800 5250 4800
Wire Wire Line
	5250 4900 4900 4900
Text Label 4950 3500 0    50   ~ 0
Out8
Text Label 4950 3600 0    50   ~ 0
Out9
Text Label 4950 3700 0    50   ~ 0
Out10
Text Label 4950 3800 0    50   ~ 0
Out11
Text Label 4950 3900 0    50   ~ 0
Out12
Text Label 4950 4000 0    50   ~ 0
Out13
Text Label 4950 4100 0    50   ~ 0
Out14
Text Label 4950 4200 0    50   ~ 0
Out15
Text Label 4950 4300 0    50   ~ 0
Out0
Text Label 4950 4400 0    50   ~ 0
Out1
Text Label 4950 4500 0    50   ~ 0
Out2
Text Label 4950 4600 0    50   ~ 0
Out3
Text Label 4950 4700 0    50   ~ 0
Out4
Text Label 4950 4800 0    50   ~ 0
Out5
Text Label 4950 4900 0    50   ~ 0
Out6
Entry Wire Line
	4800 4900 4900 5000
Wire Wire Line
	5250 5000 4900 5000
Text Label 4950 5000 0    50   ~ 0
Out7
Text Label 3950 1700 0    50   ~ 0
Out8
Text Label 3950 2200 0    50   ~ 0
Out9
Text Label 3950 2700 0    50   ~ 0
Out10
Text Label 3950 3200 0    50   ~ 0
Out11
Text Label 3950 3700 0    50   ~ 0
Out12
Text Label 3950 4200 0    50   ~ 0
Out13
Text Label 3950 4700 0    50   ~ 0
Out14
Text Label 3950 5200 0    50   ~ 0
Out15
Text Label 2350 1700 0    50   ~ 0
Out0
Text Label 2350 2200 0    50   ~ 0
Out1
Text Label 2350 2700 0    50   ~ 0
Out2
Text Label 2350 3200 0    50   ~ 0
Out3
Text Label 2350 3700 0    50   ~ 0
Out4
Text Label 2350 4200 0    50   ~ 0
Out5
Text Label 2350 4700 0    50   ~ 0
Out6
Text Label 2350 5200 0    50   ~ 0
Out7
Entry Wire Line
	2800 5100 2900 5200
Entry Wire Line
	2800 4600 2900 4700
Entry Wire Line
	2800 4100 2900 4200
Entry Wire Line
	2800 3600 2900 3700
Entry Wire Line
	2800 3100 2900 3200
Entry Wire Line
	2800 2600 2900 2700
Entry Wire Line
	2800 2100 2900 2200
Entry Wire Line
	2800 1600 2900 1700
Entry Wire Line
	1200 5100 1300 5200
Entry Wire Line
	1200 4600 1300 4700
Entry Wire Line
	1200 4100 1300 4200
Entry Wire Line
	1200 3600 1300 3700
Entry Wire Line
	1200 3100 1300 3200
Entry Wire Line
	1200 2600 1300 2700
Entry Wire Line
	1200 2100 1300 2200
Entry Wire Line
	1200 1600 1300 1700
Text Label 1350 1700 0    50   ~ 0
Pin0
Text Label 1350 2200 0    50   ~ 0
Pin1
Text Label 1350 2700 0    50   ~ 0
Pin2
Wire Bus Line
	1200 1400 2800 1400
Text Label 1350 3200 0    50   ~ 0
Pin3
Text Label 1350 3700 0    50   ~ 0
Pin4
Text Label 1350 4200 0    50   ~ 0
Pin5
Text Label 1350 4700 0    50   ~ 0
Pin6
Text Label 1350 5200 0    50   ~ 0
Pin7
Wire Wire Line
	1300 1700 1750 1700
Wire Wire Line
	1300 2200 1750 2200
Wire Wire Line
	1300 2700 1750 2700
Wire Wire Line
	1300 3200 1750 3200
Wire Wire Line
	1300 3700 1750 3700
Wire Wire Line
	1300 4200 1750 4200
Wire Wire Line
	1300 4700 1750 4700
Wire Wire Line
	1300 5200 1750 5200
Wire Wire Line
	2900 1700 3350 1700
Wire Wire Line
	2900 2200 3350 2200
Wire Wire Line
	2900 2700 3350 2700
Wire Wire Line
	2900 3200 3350 3200
Wire Wire Line
	2900 3700 3350 3700
Wire Wire Line
	2900 4200 3350 4200
Wire Wire Line
	2900 4700 3350 4700
Wire Wire Line
	2900 5200 3350 5200
Wire Bus Line
	2700 5650 4300 5650
Text Label 3050 1700 0    50   ~ 0
Pin8
Text Label 3050 2200 0    50   ~ 0
Pin9
Text Label 3050 2700 0    50   ~ 0
Pin10
Text Label 3050 3200 0    50   ~ 0
Pin11
Text Label 3050 3700 0    50   ~ 0
Pin12
Text Label 3050 4200 0    50   ~ 0
Pin13
Text Label 3050 4700 0    50   ~ 0
Pin14
Text Label 3050 5200 0    50   ~ 0
Pin15
$Comp
L Device:C C5
U 1 1 5C2C351E
P 6350 3450
AR Path="/5C242719/5C2C351E" Ref="C5"  Part="1" 
AR Path="/5C24272E/5C2C351E" Ref="C6"  Part="1" 
F 0 "C6" H 6465 3496 50  0000 L CNN
F 1 "0.1uF" H 6465 3405 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D5.0mm_W2.5mm_P2.50mm" H 6388 3300 50  0001 C CNN
F 3 "~" H 6350 3450 50  0001 C CNN
	1    6350 3450
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR048
U 1 1 5C2C35A7
P 6350 3600
AR Path="/5C242719/5C2C35A7" Ref="#PWR048"  Part="1" 
AR Path="/5C24272E/5C2C35A7" Ref="#PWR067"  Part="1" 
F 0 "#PWR067" H 6350 3350 50  0001 C CNN
F 1 "GND" H 6355 3427 50  0000 C CNN
F 2 "" H 6350 3600 50  0001 C CNN
F 3 "" H 6350 3600 50  0001 C CNN
	1    6350 3600
	1    0    0    -1  
$EndComp
Wire Wire Line
	6350 3300 6350 3250
Wire Wire Line
	6350 3250 6100 3250
Wire Bus Line
	1200 6100 1800 6100
Text Label 2850 7100 0    50   ~ 0
Pin15
Text Label 2850 7000 0    50   ~ 0
Pin14
Text Label 2850 6900 0    50   ~ 0
Pin13
Text Label 2850 6800 0    50   ~ 0
Pin12
Text Label 2850 6700 0    50   ~ 0
Pin11
Text Label 2850 6600 0    50   ~ 0
Pin10
Text Label 2850 6500 0    50   ~ 0
Pin9
Text Label 2850 6400 0    50   ~ 0
Pin8
Text Label 1500 7100 0    50   ~ 0
Pin7
Text Label 1500 7000 0    50   ~ 0
Pin6
Text Label 1500 6900 0    50   ~ 0
Pin5
Text Label 1500 6800 0    50   ~ 0
Pin4
Text Label 1500 6700 0    50   ~ 0
Pin3
Text Label 1500 6600 0    50   ~ 0
Pin2
Text Label 1500 6500 0    50   ~ 0
Pin1
Text Label 1500 6400 0    50   ~ 0
Pin0
Connection ~ 1800 6100
Wire Bus Line
	1800 6100 3150 6100
Entry Wire Line
	3050 7100 3150 7000
Entry Wire Line
	3050 7000 3150 6900
Entry Wire Line
	3050 6900 3150 6800
Entry Wire Line
	3050 6800 3150 6700
Entry Wire Line
	3050 6700 3150 6600
Entry Wire Line
	3050 6600 3150 6500
Entry Wire Line
	3050 6500 3150 6400
Entry Wire Line
	3050 6400 3150 6300
Entry Wire Line
	1700 7100 1800 7000
Entry Wire Line
	1700 7000 1800 6900
Entry Wire Line
	1700 6900 1800 6800
Entry Wire Line
	1700 6800 1800 6700
Entry Wire Line
	1700 6700 1800 6600
Entry Wire Line
	1700 6600 1800 6500
Entry Wire Line
	1700 6500 1800 6400
Entry Wire Line
	1700 6400 1800 6300
Wire Wire Line
	2700 7100 3050 7100
Wire Wire Line
	2700 7000 3050 7000
Wire Wire Line
	2700 6900 3050 6900
Wire Wire Line
	2700 6800 3050 6800
Wire Wire Line
	2700 6700 3050 6700
Wire Wire Line
	2700 6600 3050 6600
Wire Wire Line
	2700 6500 3050 6500
Wire Wire Line
	2700 6400 3050 6400
Wire Wire Line
	1350 7100 1700 7100
Wire Wire Line
	1350 7000 1700 7000
Wire Wire Line
	1350 6900 1700 6900
Wire Wire Line
	1350 6800 1700 6800
Wire Wire Line
	1350 6700 1700 6700
Wire Wire Line
	1350 6600 1700 6600
Wire Wire Line
	1350 6500 1700 6500
Wire Wire Line
	1350 6400 1700 6400
$Comp
L Connector:Conn_01x08_Male J12
U 1 1 5C265068
P 2500 6700
AR Path="/5C242719/5C265068" Ref="J12"  Part="1" 
AR Path="/5C24272E/5C265068" Ref="J14"  Part="1" 
F 0 "J14" H 2500 7150 50  0000 C CNN
F 1 "Conn_01x08_Male" H 2500 6200 50  0000 C CNN
F 2 "TerminalBlock_TE-Connectivity:TerminalBlock_TE_282834-8_1x08_P2.54mm_Horizontal" H 2500 6700 50  0001 C CNN
F 3 "~" H 2500 6700 50  0001 C CNN
	1    2500 6700
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x08_Male J11
U 1 1 5C264FE8
P 1150 6700
AR Path="/5C242719/5C264FE8" Ref="J11"  Part="1" 
AR Path="/5C24272E/5C264FE8" Ref="J13"  Part="1" 
F 0 "J13" H 1150 7150 50  0000 C CNN
F 1 "Conn_01x08_Male" H 1150 6200 50  0000 C CNN
F 2 "TerminalBlock_TE-Connectivity:TerminalBlock_TE_282834-8_1x08_P2.54mm_Horizontal" H 1150 6700 50  0001 C CNN
F 3 "~" H 1150 6700 50  0001 C CNN
	1    1150 6700
	1    0    0    -1  
$EndComp
Wire Wire Line
	6050 3700 6150 3700
Wire Wire Line
	6150 3700 6150 3950
Wire Wire Line
	6150 3950 6350 3950
Wire Bus Line
	4300 1800 4300 5650
Wire Bus Line
	2700 1800 2700 5650
Wire Bus Line
	2800 1400 2800 5100
Wire Bus Line
	3150 6100 3150 7000
Wire Bus Line
	1800 6100 1800 7000
Wire Bus Line
	1200 1400 1200 6100
Wire Bus Line
	4800 1800 4800 4900
Text HLabel 6350 3950 2    50   Input ~ 0
~RESET
$EndSCHEMATC
