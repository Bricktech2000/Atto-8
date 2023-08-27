import sys

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa


if len(sys.argv) != 3:
  print('Dec: Usage: dec <memory image file> <hex output file>')
  sys.exit(1)

memory_image_file = sys.argv[1]
hex_output_file = sys.argv[2]


with open(memory_image_file, 'rb') as memory_image_file:
  with open(hex_output_file, 'wb') as hex_output_file:
    hex_bytes = memory_image_file.read().hex(' ').split(' ')
    if len(hex_bytes) != common.MEM_SIZE:
      print(f'Dec: Error: Memory image has incorrect size')
      sys.exit(1)
    hex_output = '\n'.join(hexadecimal + ' # ' + hex(index) for (index, hexadecimal) in enumerate(hex_bytes))
    hex_output_file.write(hex_output.encode())

print('Dec: Done')
