import re
import sys
import subprocess

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('Circ')


def swap_byte_pairs(data):
  return b''.join([data[i:i+2][::-1] for i in range(0, len(data), 2)])


if len(sys.argv) != 4:
  print('Circ: Usage: circ <memory image file> <microcode image file> <circuit file>')
  sys.exit(1)

mic_label = b'<a name="label" val="MIC"/>'
rom_label = b'<a name="label" val="ROM0"/>'

memory_image_file = sys.argv[1]
microcode_image_file = sys.argv[2]
circuit_file = sys.argv[3]

with open_safe(memory_image_file, 'rb') as f:
  memory_image = f.read()
with open_safe(microcode_image_file, 'rb') as f:
  microcode_image = f.read()
with open_safe(circuit_file, 'rb') as f:
  circuit = f.read()
circuit = re.sub(rb' *<a name="contents">.*?</a>\n', b'', circuit, flags=re.DOTALL)
circuit = circuit.replace(mic_label, mic_label + b'\n<a name="contents">addr/data: 13 16\n' +
                          swap_byte_pairs(microcode_image).hex(' ', 2).encode() + b'\n</a>')
circuit = circuit.replace(rom_label, rom_label + b'\n<a name="contents">addr/data: 8 8\n' +
                          memory_image.hex(' ').encode() + b'\n</a>')
with open_safe(circuit_file, 'wb') as f:
  f.write(circuit)

try:
  print('Circ: Launching Logisim TTY...\n')
  subprocess.run(['logisim-evolution', circuit_file, '--tty', 'tty'], check=True)
except FileNotFoundError:
  print('Circ: Error: Could not find binary \'logisim-evolution\'')
  sys.exit(1)

print('Circ: Done')
