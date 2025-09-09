import sys

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('Dec')


if len(sys.argv) != 3:
  print('Dec: Usage: dec <memory image file> <hex output file>', file=sys.stderr)
  sys.exit(1)

memory_image_file = sys.argv[1]
hex_output_file = sys.argv[2]

with open_safe(memory_image_file, 'rb') as memory_image_file:
  memory_image = memory_image_file.read()

hex_bytes = memory_image.hex(' ').upper().split(' ')
if len(hex_bytes) != common.MEM_SIZE:
  print(f'Dec: Error: Memory image has incorrect size', file=sys.stderr)
  sys.exit(1)
hex_output = '\n'.join(hexadecimal + ' # ' + f'{index:02X}' for (index, hexadecimal) in enumerate(hex_bytes))
hex_output = hex_output.encode()

with open_safe(hex_output_file, 'wb') as hex_output_file:
  hex_output_file.write(hex_output)
