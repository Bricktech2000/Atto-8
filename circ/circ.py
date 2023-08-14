import re
import os
import sys
import subprocess


def swap_byte_pairs(data):
  return b''.join([data[i:i+2][::-1] for i in range(0, len(data), 2)])


if len(sys.argv) != 3:
  print('Circ: Usage: circ <memory image file> <microcode image file>')
  sys.exit(1)

circuit_file = os.path.join(os.path.dirname(__file__), 'atto-8.circ')
memory_image_file = sys.argv[1]
microcode_image_file = sys.argv[2]

with open(circuit_file, 'rb') as f:
  circ = f.read()

with open(memory_image_file, 'rb') as f:
  memory_image = f.read()

with open(microcode_image_file, 'rb') as f:
  microcode_image = f.read()


mic_label = b'<a name="label" val="MIC"/>'
preload_label = b'<a name="label" val="PRELOAD"/>'

circ = circ.replace(mic_label, mic_label + b'\n<a name="contents">addr/data: 13 16\n' +
                    swap_byte_pairs(microcode_image).hex(' ', 2).encode() + b'</a>')

circ = circ.replace(preload_label, preload_label + b'\n<a name="contents">addr/data: 8 8\n' +
                    memory_image.hex(' ').encode() + b'</a>')

with open(circuit_file, 'wb') as f:
  f.write(circ)


print('Circ: Running Logisim...')

subprocess.run(['logisim-evolution', circuit_file], check=True)

with open(circuit_file, 'rb') as f:
  circ = f.read()

circ = re.sub(rb'\n<a name="contents">.*\n[0-9a-f ]*</a>', b'', circ)

with open(circuit_file, 'wb') as f:
  f.write(circ)

print('Circ: Done')
