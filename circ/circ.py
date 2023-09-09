import re
import os
import sys
import subprocess

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('Circ')


def swap_byte_pairs(data):
  return b''.join([data[i:i+2][::-1] for i in range(0, len(data), 2)])


if len(sys.argv) != 3:
  print('Circ: Usage: circ <memory image file> <microcode image file>')
  sys.exit(1)

mic_label = b'<a name="label" val="MIC"/>'
rom_label = b'<a name="label" val="ROM0"/>'

circuit_file = os.path.join(os.path.dirname(__file__), 'atto-8.circ')
memory_image_file = sys.argv[1]
microcode_image_file = sys.argv[2]

with open_safe(circuit_file, 'rb') as f:
  circ = f.read()
with open_safe(memory_image_file, 'rb') as f:
  memory_image = f.read()
with open_safe(microcode_image_file, 'rb') as f:
  microcode_image = f.read()
circ = circ.replace(mic_label, mic_label + b'\n<a name="contents">addr/data: 13 16\n' +
                    swap_byte_pairs(microcode_image).hex(' ', 2).encode() + b'\n</a>')
circ = circ.replace(rom_label, rom_label + b'\n<a name="contents">addr/data: 8 8\n' +
                    memory_image.hex(' ').encode() + b'\n</a>')
with open_safe(circuit_file, 'wb') as f:
  f.write(circ)

print('Circ: Launching Logisim...')
subprocess.run(['logisim-evolution', circuit_file], check=True)

with open_safe(circuit_file, 'rb') as f:
  circ = f.read()
circ = re.sub(rb'<a name="contents">[^\x00]*?</a>\n', b'', circ)
with open_safe(circuit_file, 'wb') as f:
  f.write(circ)

print('Circ: Done')
