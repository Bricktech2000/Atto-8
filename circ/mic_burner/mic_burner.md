run the following to compile and upload the sketch to the Arduino Nano and burn the microcode:

```sh
arduino-cli core install arduino:avr # only required once
arduino-cli compile --fqbn arduino:avr:nano --library ../../misc/common/
arduino-cli upload --fqbn arduino:avr:nano --port /dev/ttyUSB0
python3 test.py mic pop; pv target/microcode.mic --rate-limit 75 > /dev/ttyUSB0
```

run once with switch up to burn leftmost EEPROM chip, then run again with switch down to burn rightmost EEPROM chip

make sure to add a capacitor between the RST and GND pins to prevent the Arduino Nano from resetting when the serial port is opened

**ensure builtin LED turns back on** after burning microcode; if it does not, microcode was burned incorrectly
