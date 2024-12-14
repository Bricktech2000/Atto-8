// programs `MIC` EEPROMs with data from serial. this program and this circuit
// are purpose-built for /mic/ and /circ/. write speed is bottlenecked by the
// EEPROM write delay, limiting the serial baud rate to 75 baud. the Arduino's
// builtin LED should turn back on after the burn process; if it does not, the
// microcode was not burned correctly. the switch selects which EEPROM to write
// to; the leftmost EEPROM is selected when the switch is up, and the rightmost
// EEPROM is selected when the switch is down

/* clang-format off
 * CLK_PINs -> ############74HC595############ -- ############74HC595############ -- ############74HC595############
 *  SER_PIN -> Q_A Q_B Q_C Q_D Q_E Q_F Q_G Q_H -> Q_A Q_B Q_C Q_D Q_E Q_F Q_G Q_H -> Q_A Q_B Q_C Q_D Q_E Q_F Q_G Q_H
 *              V   V   V   V   V   V   V   V      V   V   V   V   V   V   V   V      V   V   V   V   V   V   V   V
 *                         A6  A12 A3  A2  A1     A10 A4  A5  A9  A8  A11 A0  A7     IO0 IO1 IO2 IO7 IO6 IO5 IO4 IO3
 * N_WE_PIN -> ##############################################AT28C64B###############################################
 */

#include "common.h"

#define SER_PIN 2   // 74HC595 shift register data pin
#define SRCLK_PIN 3 // 74HC595 shift register clock pin
#define RCLK_PIN 4  // 74HC595 shift register latch clock pin
#define N_WE_PIN 5  // AT28C64B EEPROM active-low write enable pin

#define SW_PIN 6 // toggle switch pin

void eeprom_write(uint16_t addr, uint8_t data) {
  shiftOut(SER_PIN, SRCLK_PIN, LSBFIRST, data);
  shiftOut(SER_PIN, SRCLK_PIN, LSBFIRST, addr & 0xFF);
  shiftOut(SER_PIN, SRCLK_PIN, LSBFIRST, addr >> 8);

  digitalWrite(RCLK_PIN, HIGH);
  digitalWrite(RCLK_PIN, LOW);

  digitalWrite(N_WE_PIN, LOW);
  delayMicroseconds(10);
  digitalWrite(N_WE_PIN, HIGH);
  delay(12);
}

void setup(void) {
  digitalWrite(RCLK_PIN, LOW);
  digitalWrite(N_WE_PIN, HIGH);

  pinMode(SER_PIN, OUTPUT);
  pinMode(SRCLK_PIN, OUTPUT);
  pinMode(RCLK_PIN, OUTPUT);
  pinMode(N_WE_PIN, OUTPUT);

  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(SW_PIN, INPUT_PULLUP);

  Serial.begin(9600);
}

uint16_t addr = 0x0000;

void loop(void) {
  digitalWrite(LED_BUILTIN, addr == 0x0000);

  while (Serial.available() < 2)
    ;

  int sw_up = !digitalRead(SW_PIN);
  uint8_t lsb = Serial.read();
  uint8_t msb = Serial.read();
  uint8_t data = sw_up ? (msb >> 3) | (msb << 5) // leftmost EEPROM
                       : lsb ^ 0b11111111;       // rightmost EEPROM

  eeprom_write(addr++ ^ 0b1110000000000, data);
  addr &= MIC_SIZE - 1; // 13-bit address space
}
